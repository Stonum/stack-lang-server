//! PL/pgSQL bodies of `DO` blocks, parsed in the same tree as SQL.

use biome_parser::ParserProgress;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use crate::SqlParser;
use crate::lexer::SqlReLexContext;
use crate::syntax_rules::parse_error::expected_statement;
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

/// `BEGIN statement; ... END [;]`
fn parse_pl_block(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![begin]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![begin]);
    parse_pl_statement_list(p);
    p.expect(T![end]);
    p.eat(T![;]);
    Present(m.complete(p, PSQL_PL_BLOCK))
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
    if p.at(T![begin]) {
        return parse_pl_block(p);
    }
    parse_statement(p)
}

fn expected_pl_block(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a `begin ... end` block", range)
}
