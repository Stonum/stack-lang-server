//! PL/pgSQL bodies of `DO` blocks, parsed in the same tree as SQL.

use biome_parser::ParserProgress;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use crate::SqlParser;
use crate::lexer::SqlReLexContext;
use crate::syntax_rules::expr::{
    is_at_name_start, parse_binary_or_logical_expression, parse_expression, parse_name,
    parse_type_name,
};
use crate::syntax_rules::parse_error::{
    expected_expression, expected_identifier, expected_statement, expected_type_name,
};
use crate::syntax_rules::stmt::{STMT_RECOVERY_SET, parse_statement};
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

/// `[DECLARE declaration; ...] BEGIN statement; ... END [;]`
fn parse_pl_block(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![begin]) && !p.at(T![declare]) {
        return Absent;
    }

    let m = p.start();
    let _ = parse_pl_declare_section(p);
    p.expect(T![begin]);
    parse_pl_statement_list(p);
    p.expect(T![end]);
    p.eat(T![;]);
    Present(m.complete(p, PSQL_PL_BLOCK))
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
    if p.at(T![if]) {
        return parse_pl_if_statement(p);
    }
    if is_at_pl_assignment(p) {
        return parse_pl_assignment_statement(p);
    }
    parse_statement(p)
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

fn expected_pl_block(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a `begin ... end` block", range)
}
