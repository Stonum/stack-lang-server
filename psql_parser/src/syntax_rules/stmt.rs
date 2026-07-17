use biome_parser::ParserProgress;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::delete::{parse_delete_statement, parse_delete_statement_body};
use super::insert::{parse_insert_statement, parse_insert_statement_body};
use super::parse_error::*;
use super::select::{parse_select_statement, parse_select_statement_body};
use super::update::{parse_update_statement, parse_update_statement_body};
use super::with_clause::parse_with_clause;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

pub const STMT_RECOVERY_SET: TokenSet<PsqlSyntaxKind> = token_set![T![;]];

pub(crate) fn parse_statements(p: &mut PsqlParser, statement_list: Marker) {
    let mut progress = ParserProgress::default();

    while !p.at(EOF) {
        progress.assert_progressing(p);

        if parse_statement(p)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(PSQL_BOGUS_STATEMENT, STMT_RECOVERY_SET),
                expected_statement,
            )
            .is_err()
        {
            break;
        }
    }

    statement_list.complete(p, PSQL_STATEMENT_LIST);
}

pub(crate) fn parse_statement(p: &mut PsqlParser) -> ParsedSyntax {
    // A stray `;` (e.g. the second `;` in `select 1;; select 2`) is a
    // harmless empty statement, not an error -- most SQL implementations
    // treat it as a no-op. Handled as a real `AnyPsqlStatement` alternative
    // (`PsqlEmptyStatement`) rather than skipped ad hoc: `;` is both what
    // statement-recovery skips *to* and what it recovers *from*, so treating
    // it as "not a statement" here would make recovery immediately hit
    // "already at a recovery token" and abort the whole statement list.
    if p.at(T![;]) {
        let m = p.start();
        p.bump(T![;]);
        return Present(m.complete(p, PSQL_EMPTY_STATEMENT));
    }

    if p.at(T![with]) {
        return parse_with_prefixed_statement(p);
    }

    match p.cur() {
        T![select] => parse_select_statement(p),
        T![delete] => parse_delete_statement(p),
        T![update] => parse_update_statement(p),
        T![insert] => parse_insert_statement(p),
        _ => Absent,
    }
}

/// Parses a leading `with [recursive] cte as (...), ...` clause once, then
/// dispatches to whichever statement body follows it (`select`/`insert`/
/// `update`/`delete`), handing over the marker so the `with` clause becomes
/// that statement's first child.
fn parse_with_prefixed_statement(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    let _ = parse_with_clause(p);

    match p.cur() {
        T![select] => parse_select_statement_body(p, m),
        T![insert] => parse_insert_statement_body(p, m),
        T![update] => parse_update_statement_body(p, m),
        T![delete] => parse_delete_statement_body(p, m),
        _ => {
            let range = p.cur_range();
            let err = p
                .err_builder(
                    "Expected `select`, `insert`, `update` or `delete` after a `with` clause",
                    range,
                )
                .with_hint("A `with` clause must be followed by a statement");
            p.error(err);
            Present(m.complete(p, PSQL_BOGUS_STATEMENT))
        }
    }
}
