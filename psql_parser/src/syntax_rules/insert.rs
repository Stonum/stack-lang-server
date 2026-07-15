use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{EXPR_RECOVERY_SET, PsqlExpressionList, parse_name};
use super::from::parse_table_binding;
use super::parse_error::*;
use super::select::parse_select_statement;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

pub(crate) fn parse_insert_statement(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![insert]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![insert]);
    p.expect(T![into]);
    parse_table_binding(p).or_add_diagnostic(p, expected_table_binding);

    let _ = parse_insert_columns(p);
    parse_insert_source(p).or_add_diagnostic(p, expected_insert_source);
    p.eat(T![;]);

    Present(m.complete(p, PSQL_INSERT_STATEMENT))
}

fn parse_insert_columns(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    PsqlInsertColumnList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_INSERT_COLUMNS))
}

struct PsqlInsertColumnList;

impl ParseSeparatedList for PsqlInsertColumnList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_INSERT_COLUMN_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_name(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS, EXPR_RECOVERY_SET),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

/// The source of the inserted rows: either `VALUES (...)` or a `SELECT`
/// statement (`INSERT INTO t SELECT ...`).
fn parse_insert_source(p: &mut PsqlParser) -> ParsedSyntax {
    match p.cur() {
        T![values] => parse_insert_values(p),
        T![select] => parse_select_statement(p),
        _ => Absent,
    }
}

fn parse_insert_values(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![values]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![values]);
    p.expect(T!['(']);
    PsqlExpressionList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_INSERT_VALUES))
}
