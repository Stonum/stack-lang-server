//! PL/pgSQL bodies of `DO` blocks, parsed in the same tree as SQL.

use biome_parser::ParserProgress;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use crate::SqlParser;
use crate::lexer::SqlReLexContext;
use crate::syntax_rules::expr::{is_at_name_start, parse_expression, parse_name, parse_type_name};
use crate::syntax_rules::parse_error::{
    expected_expression, expected_identifier, expected_statement, expected_type_name,
};
use crate::syntax_rules::stmt::{STMT_RECOVERY_SET, parse_statement};
use sql_syntax::{SqlSyntaxKind, SqlSyntaxKind::*, T, TextRange};

const PL_STATEMENT_RECOVERY_SET: TokenSet<SqlSyntaxKind> =
    STMT_RECOVERY_SET.union(token_set![T![end], DOLLAR_QUOTE]);

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

    while !p.at(EOF) && !p.at(T![end]) && !p.at(DOLLAR_QUOTE) {
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
    parse_statement(p)
}

fn expected_declaration(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a variable declaration", range)
}

fn expected_pl_block(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a `begin ... end` block", range)
}
