use biome_parser::ParserProgress;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::Absent;
use biome_parser::prelude::*;

use super::parse_error::*;
use super::select::parse_select_statement;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

pub const STMT_RECOVERY_SET: TokenSet<PsqlSyntaxKind> = token_set![T![;]];

pub(crate) fn parse_statements(p: &mut PsqlParser, statement_list: Marker) {
    let mut progress = ParserProgress::default();

    while !p.at(EOF) {
        progress.assert_progressing(p);

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
    match p.cur() {
        T![select] => parse_select_statement(p),
        // T![insert] => parse_insert_statement(p),
        // T![update] => parse_update_statement(p),
        // T![delete] => parse_delete_statement(p),
        _ => Absent,
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
