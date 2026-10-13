//! PL/pgSQL bodies of `DO` blocks, parsed in the same tree as SQL.

use biome_parser::ParserProgress;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use crate::SqlParser;
use crate::lexer::SqlReLexContext;
use crate::syntax_rules::expr::{at_subquery, parse_subquery_expression};
use crate::syntax_rules::expr::{
    is_at_name_start, parse_binary_or_logical_expression, parse_expression, parse_label,
    parse_name, parse_type_name,
};
use crate::syntax_rules::from::parse_from_clause;
use crate::syntax_rules::parse_error::{
    expected_expression, expected_identifier, expected_statement, expected_type_name,
};
use crate::syntax_rules::select::{
    SqlSelectItemList, UnbracketedExpressionList, is_at_parenthesized_set_operand,
    parse_into_clause,
};
use crate::syntax_rules::stmt::{STMT_RECOVERY_SET, parse_statement};
use crate::syntax_rules::where_clause::parse_where_clause;
use sql_syntax::{OperatorPrecedence, SqlSyntaxKind, SqlSyntaxKind::*, T, TextRange};

/// Tokens that close a statement list: the block's `END` or a branch of an
/// enclosing `IF`.
const PL_STATEMENT_LIST_END_SET: TokenSet<SqlSyntaxKind> =
    token_set![T![end], T![else], T![elsif], T![elseif], DOLLAR_QUOTE, EOF];

const PL_STATEMENT_RECOVERY_SET: TokenSet<SqlSyntaxKind> =
    STMT_RECOVERY_SET.union(PL_STATEMENT_LIST_END_SET);

/// Whether a routine body in `language` is PL/pgSQL; `None` (no `LANGUAGE`
/// clause) means the `DO` default, which is PL/pgSQL.
pub(crate) fn is_plpgsql_language(language: Option<&str>) -> bool {
    language.is_none_or(|language| {
        language
            .trim_matches(|c| c == '\'' || c == '"')
            .eq_ignore_ascii_case("plpgsql")
    })
}

/// `$tag$ BEGIN ... END $tag$` -- re-lexes the current dollar-quoted string
/// token as code. Absent (the token stays a string) for any other literal.
pub(crate) fn parse_pl_body(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(SQL_STRING_LITERAL) || !p.text(p.cur_range()).starts_with('$') {
        return Absent;
    }
    if p.re_lex(SqlReLexContext::DollarQuotedBody) != DOLLAR_QUOTE {
        return Absent;
    }

    let m = p.start();
    p.bump(DOLLAR_QUOTE);
    parse_pl_block(p).or_add_diagnostic(p, expected_pl_block);
    if !p.at(DOLLAR_QUOTE) {
        let range = p.cur_range();
        let err = p.err_builder("Expected the end of the block body", range);
        p.error(err);
        while !p.at(DOLLAR_QUOTE) && !p.at(EOF) {
            p.bump_any();
        }
    }
    p.eat(DOLLAR_QUOTE);
    Present(m.complete(p, PSQL_PL_BODY))
}

/// `[<<label>>] [DECLARE declaration; ...] BEGIN statement; ... END [label] [;]`
fn parse_pl_block(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![begin]) && !p.at(T![declare]) && !p.at(T![<<]) {
        return Absent;
    }

    let m = p.start();
    let label = parse_pl_label(p);
    parse_pl_block_rest(p, m, label.as_deref())
}

/// The block after its optional `label`, which `m` already holds.
fn parse_pl_block_rest(p: &mut SqlParser, m: Marker, label: Option<&str>) -> ParsedSyntax {
    let _ = parse_pl_declare_section(p);
    p.expect(T![begin]);
    parse_pl_statement_list(p);
    p.expect(T![end]);
    parse_pl_end_label(p, label);
    p.eat(T![;]);
    Present(m.complete(p, PSQL_PL_BLOCK))
}

/// `<<name>>`; returns the label's name.
fn parse_pl_label(p: &mut SqlParser) -> Option<String> {
    if !p.at(T![<<]) {
        return None;
    }

    let m = p.start();
    p.bump(T![<<]);
    let name = is_at_name_start(p).then(|| p.text(p.cur_range()).to_string());
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    p.expect(T![>>]);
    m.complete(p, PSQL_PL_LABEL);
    name
}

/// A label name after `EXIT`/`CONTINUE`.
fn parse_pl_label_reference(p: &mut SqlParser) -> ParsedSyntax {
    if p.at(T![when]) {
        return Absent;
    }
    parse_name(p)
}

/// The optional label repeated after `END [LOOP]`, which must match the
/// opening `label`.
fn parse_pl_end_label(p: &mut SqlParser, label: Option<&str>) {
    if !is_at_name_start(p) {
        return;
    }

    let range = p.cur_range();
    let end_label = p.text(range).to_string();
    let _ = parse_name(p);
    match label {
        Some(label) if label.eq_ignore_ascii_case(&end_label) => {}
        Some(label) => {
            let err = p.err_builder(
                format!("End label `{end_label}` differs from the label `{label}`"),
                range,
            );
            p.error(err);
        }
        None => {
            let err = p.err_builder(
                format!("End label `{end_label}` specified for an unlabelled block"),
                range,
            );
            p.error(err);
        }
    }
}

/// A labelled statement: `<<name>>` followed by a block or a loop.
fn parse_pl_labelled_statement(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    let label = parse_pl_label(p);
    if p.at(T![loop]) {
        parse_pl_loop_body(p, label.as_deref()).ok();
        return Present(m.complete(p, PSQL_PL_LOOP_STATEMENT));
    }
    if p.at(T![while]) {
        return parse_pl_while_rest(p, m, label.as_deref());
    }
    if p.at(T![for]) {
        return parse_pl_for_rest(p, m, label.as_deref());
    }
    if p.at(T![foreach]) {
        return parse_pl_foreach_rest(p, m, label.as_deref());
    }
    parse_pl_block_rest(p, m, label.as_deref())
}

/// `FOREACH target, ... [SLICE n] IN ARRAY expr LOOP ... END LOOP [label];`
/// after its optional `label`, which `m` already holds.
fn parse_pl_foreach_rest(p: &mut SqlParser, m: Marker, label: Option<&str>) -> ParsedSyntax {
    p.expect(T![foreach]);
    parse_pl_for_targets(p);
    if p.at(T![slice]) {
        let slice = p.start();
        p.bump(T![slice]);
        // Stops before `in`, which belongs to `FOREACH`, not to a predicate.
        parse_binary_or_logical_expression(p, OperatorPrecedence::Predicate)
            .or_add_diagnostic(p, expected_expression);
        slice.complete(p, PSQL_PL_FOREACH_SLICE);
    }
    p.expect(T![in]);
    p.expect(T![array]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    parse_pl_loop_body(p, label).or_add_diagnostic(p, expected_loop_body);
    Present(m.complete(p, PSQL_PL_FOREACH_STATEMENT))
}

/// `name, ...` -- the loop variables of `FOR`/`FOREACH`.
fn parse_pl_for_targets(p: &mut SqlParser) {
    let targets = p.start();
    loop {
        parse_name(p).or_add_diagnostic(p, expected_identifier);
        if !p.eat(T![,]) {
            break;
        }
    }
    targets.complete(p, PSQL_PL_FOR_TARGET_LIST);
}

/// A query start: `select`, `with`, `values`, `table` or `(query)`.
fn is_at_for_query(p: &mut SqlParser) -> bool {
    p.at(T![select])
        || p.at(T![with])
        || p.at(T![values])
        || p.at(T![table])
        || (p.at(T!['(']) && at_subquery(p))
}

/// `[REVERSE] lower .. upper [BY step]`
fn parse_pl_for_range_source(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.eat(T![reverse]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    p.expect(T![..]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    if p.at(T![by]) {
        let step = p.start();
        p.bump(T![by]);
        parse_expression(p).or_add_diagnostic(p, expected_expression);
        step.complete(p, PSQL_PL_FOR_RANGE_STEP);
    }
    Present(m.complete(p, PSQL_PL_FOR_RANGE_SOURCE))
}

/// `FOR target, ... IN source LOOP ... END LOOP [label];` after its optional
/// `label`, which `m` already holds.
fn parse_pl_for_rest(p: &mut SqlParser, m: Marker, label: Option<&str>) -> ParsedSyntax {
    p.expect(T![for]);
    parse_pl_for_targets(p);

    p.expect(T![in]);
    parse_pl_for_source(p).or_add_diagnostic(p, expected_for_source);
    parse_pl_loop_body(p, label).or_add_diagnostic(p, expected_loop_body);
    Present(m.complete(p, PSQL_PL_FOR_STATEMENT))
}

/// A query, `(query)` or `EXECUTE command [USING ...]`, ending before `LOOP`.
fn parse_pl_for_source(p: &mut SqlParser) -> ParsedSyntax {
    if p.at(T![execute]) {
        let m = p.start();
        p.bump(T![execute]);
        parse_expression(p).or_add_diagnostic(p, expected_expression);
        if p.at(T![using]) {
            let using = p.start();
            p.bump(T![using]);
            let arguments = UnbracketedExpressionList.parse_list(p);
            if arguments.range(p).is_empty() {
                let range = p.cur_range();
                p.error(expected_expression(p, range));
            }
            using.complete(p, PSQL_PL_USING_CLAUSE);
        }
        return Present(m.complete(p, PSQL_PL_FOR_EXECUTE_SOURCE));
    }
    if p.at(T![loop]) {
        return Absent;
    }
    if p.at(T![reverse]) || !is_at_for_query(p) {
        return parse_pl_for_range_source(p);
    }

    let outer = p.in_for_loop_query;
    p.in_for_loop_query = true;
    let source = if p.at(T!['(']) && !is_at_parenthesized_set_operand(p) {
        parse_subquery_expression(p)
    } else {
        parse_statement(p)
    };
    p.in_for_loop_query = outer;
    source
}

/// `WHILE cond LOOP ... END LOOP [label];` after its optional `label`,
/// which `m` already holds.
fn parse_pl_while_rest(p: &mut SqlParser, m: Marker, label: Option<&str>) -> ParsedSyntax {
    p.expect(T![while]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    parse_pl_loop_body(p, label).or_add_diagnostic(p, expected_loop_body);
    Present(m.complete(p, PSQL_PL_WHILE_STATEMENT))
}

/// `LOOP statement; ... END LOOP [label];`
fn parse_pl_loop_body(p: &mut SqlParser, label: Option<&str>) -> ParsedSyntax {
    if !p.at(T![loop]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![loop]);
    parse_pl_statement_list(p);
    p.expect(T![end]);
    p.expect(T![loop]);
    parse_pl_end_label(p, label);
    p.expect(T![;]);
    Present(m.complete(p, PSQL_PL_LOOP_BODY))
}

/// `{EXIT | CONTINUE} [label] [WHEN cond];`
fn parse_pl_exit_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![exit]) && !p.at(T![continue]) {
        return Absent;
    }

    let m = p.start();
    p.bump_any();
    let _ = parse_pl_label_reference(p);
    if p.at(T![when]) {
        let when = p.start();
        p.bump(T![when]);
        parse_expression(p).or_add_diagnostic(p, expected_expression);
        when.complete(p, PSQL_PL_EXIT_WHEN_CLAUSE);
    }
    p.expect(T![;]);
    Present(m.complete(p, PSQL_PL_EXIT_STATEMENT))
}

fn parse_pl_declare_section(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![declare]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![declare]);

    let list = p.start();
    let mut progress = ParserProgress::default();
    while !p.at(EOF) && !p.at(T![begin]) && !p.at(DOLLAR_QUOTE) {
        progress.assert_progressing(p);

        if parse_pl_variable_declaration(p)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(SQL_BOGUS, token_set![T![;], T![begin], DOLLAR_QUOTE]),
                expected_declaration,
            )
            .is_err()
        {
            break;
        }
        // A broken declaration leaves its `;` behind.
        p.eat(T![;]);
    }
    list.complete(p, PSQL_PL_DECLARATION_LIST);

    Present(m.complete(p, PSQL_PL_DECLARE_SECTION))
}

/// `name [CONSTANT] type [COLLATE c] [NOT NULL] [{DEFAULT | := | =} value];`
fn parse_pl_variable_declaration(p: &mut SqlParser) -> ParsedSyntax {
    if !is_at_name_start(p) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    p.eat(T![constant]);
    parse_type_name(p).or_add_diagnostic(p, expected_type_name);

    if p.at(T![collate]) {
        let collation = p.start();
        p.bump(T![collate]);
        parse_name(p).or_add_diagnostic(p, expected_identifier);
        collation.complete(p, SQL_COLLATE_CONSTRAINT);
    }
    if p.at(T![not]) && p.nth_at(1, T![null]) {
        let not_null = p.start();
        p.bump(T![not]);
        p.bump(T![null]);
        not_null.complete(p, SQL_NOT_NULL_CONSTRAINT);
    }
    if p.at(T![default]) || p.at(T![:=]) || p.at(T![=]) {
        let default = p.start();
        p.bump_any();
        parse_expression(p).or_add_diagnostic(p, expected_expression);
        default.complete(p, PSQL_PL_VARIABLE_DEFAULT);
    }

    p.expect(T![;]);
    Present(m.complete(p, PSQL_PL_VARIABLE_DECLARATION))
}

fn parse_pl_statement_list(p: &mut SqlParser) {
    let list = p.start();
    let mut progress = ParserProgress::default();

    while !p.at_ts(PL_STATEMENT_LIST_END_SET) {
        progress.assert_progressing(p);

        if parse_pl_statement(p)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(SQL_BOGUS_STATEMENT, PL_STATEMENT_RECOVERY_SET),
                expected_statement,
            )
            .is_err()
        {
            break;
        }
    }

    list.complete(p, PSQL_PL_STATEMENT_LIST);
}

fn parse_pl_statement(p: &mut SqlParser) -> ParsedSyntax {
    // Inside a body `begin` opens a nested block, not a transaction.
    if p.at(T![begin]) || p.at(T![declare]) {
        return parse_pl_block(p);
    }
    if p.at(T![<<]) {
        return parse_pl_labelled_statement(p);
    }
    if p.at(T![loop]) {
        let m = p.start();
        parse_pl_loop_body(p, None).ok();
        return Present(m.complete(p, PSQL_PL_LOOP_STATEMENT));
    }
    if p.at(T![while]) {
        let m = p.start();
        return parse_pl_while_rest(p, m, None);
    }
    if p.at(T![for]) {
        let m = p.start();
        return parse_pl_for_rest(p, m, None);
    }
    if p.at(T![foreach]) {
        let m = p.start();
        return parse_pl_foreach_rest(p, m, None);
    }
    if p.at(T![exit]) || p.at(T![continue]) {
        return parse_pl_exit_statement(p);
    }
    if p.at(T![get])
        && (p.nth_at(1, T![diagnostics]) || p.nth_at(1, T![current]) || p.nth_at(1, T![stacked]))
    {
        return parse_pl_get_diagnostics_statement(p);
    }
    if p.at(T![if]) {
        return parse_pl_if_statement(p);
    }
    if p.at(T![perform]) {
        return parse_pl_perform_statement(p);
    }
    if p.at(T![execute]) {
        return parse_pl_execute_statement(p);
    }
    if p.at(T![raise]) {
        return parse_pl_raise_statement(p);
    }
    if p.at(T![return]) {
        return parse_pl_return_statement(p);
    }
    if is_at_pl_assignment(p) {
        return parse_pl_assignment_statement(p);
    }
    parse_statement(p)
}

/// `EXECUTE command [INTO [STRICT] target, ...] [USING expr, ...];`
fn parse_pl_execute_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![execute]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![execute]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    let _ = parse_into_clause(p);
    if p.at(T![using]) {
        let using = p.start();
        p.bump(T![using]);
        let arguments = UnbracketedExpressionList.parse_list(p);
        if arguments.range(p).is_empty() {
            let range = p.cur_range();
            p.error(expected_expression(p, range));
        }
        using.complete(p, PSQL_PL_USING_CLAUSE);
    }
    p.expect(T![;]);
    Present(m.complete(p, PSQL_PL_EXECUTE_STATEMENT))
}

const RAISE_LEVEL_SET: TokenSet<SqlSyntaxKind> = token_set![
    T![debug],
    T![log],
    T![info],
    T![notice],
    T![warning],
    T![exception]
];

/// `RAISE [level] ['format', arg, ... | condition | SQLSTATE 'code']
/// [USING option = value, ...];`
fn parse_pl_raise_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![raise]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![raise]);
    p.eat_ts(RAISE_LEVEL_SET);
    let sqlstate = p.eat(T![sqlstate]);
    let arguments = UnbracketedExpressionList.parse_list(p);
    if sqlstate && arguments.range(p).is_empty() {
        let range = p.cur_range();
        p.error(expected_expression(p, range));
    }

    if p.at(T![using]) {
        let using = p.start();
        p.bump(T![using]);
        let options = p.start();
        let mut progress = ParserProgress::default();
        loop {
            progress.assert_progressing(p);
            if parse_pl_raise_option(p)
                .or_recover_with_token_set(
                    p,
                    &ParseRecoveryTokenSet::new(
                        SQL_BOGUS,
                        PL_STATEMENT_RECOVERY_SET.union(token_set![T![,], T![;]]),
                    ),
                    expected_raise_option,
                )
                .is_err()
                || !p.eat(T![,])
            {
                break;
            }
        }
        options.complete(p, PSQL_PL_RAISE_OPTION_LIST);
        using.complete(p, PSQL_PL_RAISE_USING_CLAUSE);
    }

    p.expect(T![;]);
    Present(m.complete(p, PSQL_PL_RAISE_STATEMENT))
}

/// `name = value`; option names (`column`, `table`, ...) may be keywords.
fn parse_pl_raise_option(p: &mut SqlParser) -> ParsedSyntax {
    if !is_at_name_start(p) && !p.cur().is_keyword() {
        return Absent;
    }

    let m = p.start();
    parse_label(p).or_add_diagnostic(p, expected_identifier);
    p.expect(T![=]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_PL_RAISE_OPTION))
}

/// `RETURN [NEXT] [expr];` or `RETURN QUERY {query | EXECUTE ...};`
fn parse_pl_return_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![return]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![return]);
    if p.eat(T![query]) {
        if p.at(T![;]) {
            let range = p.cur_range();
            p.error(expected_statement(p, range));
            p.bump(T![;]);
        } else if p.at(T![execute]) {
            parse_pl_execute_statement(p).or_add_diagnostic(p, expected_statement);
        } else {
            parse_statement(p).or_add_diagnostic(p, expected_statement);
        }
        return Present(m.complete(p, PSQL_PL_RETURN_QUERY_STATEMENT));
    }

    p.eat(T![next]);
    if !p.at(T![;]) {
        parse_expression(p).or_add_diagnostic(p, expected_expression);
    }
    p.expect(T![;]);
    Present(m.complete(p, PSQL_PL_RETURN_STATEMENT))
}

/// `GET [CURRENT | STACKED] DIAGNOSTICS target = item, ...;`
fn parse_pl_get_diagnostics_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![get]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![get]);
    if p.at(T![current]) || p.at(T![stacked]) {
        p.bump_any();
    }
    p.expect(T![diagnostics]);

    let items = p.start();
    loop {
        let item = p.start();
        parse_name(p).or_add_diagnostic(p, expected_identifier);
        if p.at(T![=]) || p.at(T![:=]) {
            p.bump_any();
        } else {
            let range = p.cur_range();
            let err = p.err_builder("Expected `=` or `:=`", range);
            p.error(err);
        }
        parse_label(p).or_add_diagnostic(p, expected_identifier);
        item.complete(p, PSQL_PL_DIAGNOSTICS_ITEM);
        if !p.eat(T![,]) {
            break;
        }
    }
    items.complete(p, PSQL_PL_DIAGNOSTICS_ITEM_LIST);

    p.expect(T![;]);
    Present(m.complete(p, PSQL_PL_GET_DIAGNOSTICS_STATEMENT))
}

/// `PERFORM items [FROM ...] [WHERE ...];`
fn parse_pl_perform_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![perform]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![perform]);
    let items = SqlSelectItemList.parse_list(p);
    if items.range(p).is_empty() {
        let range = p.cur_range();
        p.error(expected_expression(p, range));
    }
    let _ = parse_from_clause(p);
    let _ = parse_where_clause(p);
    p.expect(T![;]);
    Present(m.complete(p, PSQL_PL_PERFORM_STATEMENT))
}

/// `IF cond THEN ... [ELSIF cond THEN ...] [ELSE ...] END IF;`
fn parse_pl_if_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![if]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![if]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    p.expect(T![then]);
    parse_pl_statement_list(p);

    let elsif_clauses = p.start();
    while p.at(T![elsif]) || p.at(T![elseif]) {
        let clause = p.start();
        p.bump_any();
        parse_expression(p).or_add_diagnostic(p, expected_expression);
        p.expect(T![then]);
        parse_pl_statement_list(p);
        clause.complete(p, PSQL_PL_ELSIF_CLAUSE);
    }
    elsif_clauses.complete(p, PSQL_PL_ELSIF_CLAUSE_LIST);

    if p.at(T![else]) {
        let clause = p.start();
        p.bump(T![else]);
        parse_pl_statement_list(p);
        clause.complete(p, PSQL_PL_ELSE_CLAUSE);
    }

    p.expect(T![end]);
    p.expect(T![if]);
    p.expect(T![;]);
    Present(m.complete(p, PSQL_PL_IF_STATEMENT))
}

/// A name, optionally followed by `.field`/`[index]` segments, then `:=`/`=`.
fn is_at_pl_assignment(p: &mut SqlParser) -> bool {
    if !is_at_name_start(p) {
        return false;
    }
    p.lookahead(|p| {
        p.bump_any();
        loop {
            if p.at(T![.]) {
                p.bump(T![.]);
                if p.at(EOF) {
                    return false;
                }
                p.bump_any();
            } else if p.at(T!['[']) {
                let mut depth = 0usize;
                loop {
                    match p.cur() {
                        T!['['] => depth += 1,
                        T![']'] => {
                            depth -= 1;
                            if depth == 0 {
                                p.bump_any();
                                break;
                            }
                        }
                        EOF => return false,
                        _ => {}
                    }
                    p.bump_any();
                }
            } else {
                return p.at(T![:=]) || p.at(T![=]);
            }
        }
    })
}

/// `target {:= | =} value;`
fn parse_pl_assignment_statement(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    // Stops before `=`, which is the assignment here, not a comparison.
    parse_binary_or_logical_expression(p, OperatorPrecedence::Equality)
        .or_add_diagnostic(p, expected_expression);
    if p.at(T![:=]) || p.at(T![=]) {
        p.bump_any();
    } else {
        let range = p.cur_range();
        let err = p.err_builder("Expected `:=` or `=`", range);
        p.error(err);
    }
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    p.expect(T![;]);
    Present(m.complete(p, PSQL_PL_ASSIGNMENT_STATEMENT))
}

fn expected_declaration(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a variable declaration", range)
}

fn expected_raise_option(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a `name = value` option", range)
}

fn expected_for_source(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a query or `execute ...`", range)
}

fn expected_loop_body(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected `loop ... end loop`", range)
}

fn expected_pl_block(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a `begin ... end` block", range)
}
