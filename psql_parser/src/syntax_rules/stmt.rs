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

        // A stray `;` (e.g. `select 1;; select 2`, or whatever's left right
        // before `;` once statement-recovery has skipped over unparseable
        // content up to it) is a harmless empty statement, not an error --
        // skip it directly rather than trying to recover from it. Recovery
        // itself can't handle this case: `;` is *both* what recovery skips
        // to *and* what it's recovering from, so retrying `parse_statement`
        // on it immediately hits "already at a recovery token" and aborts
        // the whole statement list, silently dropping everything after it.
        if p.at(T![;]) {
            p.bump(T![;]);
            continue;
        }

        if parse_statement(p, StatementContext::StatementList)
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

pub(crate) fn parse_statement(p: &mut PsqlParser, _context: StatementContext) -> ParsedSyntax {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum StatementContext {
    StatementList,
}

impl StatementContext {
    pub(crate) fn is_single_statement(&self) -> bool {
        !matches!(self, StatementContext::StatementList)
    }
}
