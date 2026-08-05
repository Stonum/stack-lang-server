use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::SqlExpressionList;
use super::parse_error::*;
use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind::*, T, *};

/// `VALUES (...), (...), ...` -- real Postgres allows this wherever a
/// `SELECT` can appear (a bare top-level statement, a subquery/derived
/// table, a CTE body), not just as `INSERT`'s own source, so this is its
/// own shared parser rather than living in `insert.rs`.
pub(crate) fn parse_values_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![values]) {
        return Absent;
    }

    let m = p.start();
    parse_values_clause_body(p, m)
}

/// Assumes an optional leading `with` clause has already been parsed (or
/// intentionally omitted) into `m` by the caller -- mirrors
/// `select.rs::parse_select_statement_body`'s split, needed for the exact
/// same reason (`with_clause.rs::parse_with_prefixed_select_statement` and
/// `stmt.rs::parse_with_prefixed_statement` both open the marker themselves
/// before dispatching on `select` vs `values`).
pub(crate) fn parse_values_clause_body(p: &mut SqlParser, m: Marker) -> ParsedSyntax {
    p.bump(T![values]);
    SqlValuesRowList.parse_list(p);
    p.eat(T![;]);
    Present(m.complete(p, SQL_VALUES_CLAUSE))
}

struct SqlValuesRowList;

impl ParseSeparatedList for SqlValuesRowList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_VALUES_ROW_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_values_row(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // `;` ends a top-level statement; `)` ends a nested subquery/CTE
        // body; `on`/`returning` end an `INSERT ... VALUES (...)` source
        // (`ON CONFLICT`/`RETURNING` follow directly, this being the
        // insert's source rather than a standalone statement).
        p.at(EOF) || p.at(T![;]) || p.at(T![')']) || p.at(T![on]) || p.at(T![returning])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SQL_BOGUS, token_set![T![')'], T![;]]),
            expected_values_row,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_values_row(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    SqlExpressionList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, SQL_VALUES_ROW))
}
