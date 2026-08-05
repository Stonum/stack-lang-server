use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{EXPR_RECOVERY_SET, is_at_name_start, parse_expression, parse_name};
use super::from::{SqlFromItemList, parse_table_binding};
use super::parse_error::*;
use super::returning_clause::parse_returning_clause;
use super::where_clause::parse_where_clause;
use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind::*, T, *};

pub(crate) fn parse_update_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![update]) {
        return Absent;
    }

    let m = p.start();
    parse_update_statement_body(p, m)
}

/// Parses the body of an `update` statement, assuming an optional leading
/// `with` clause has already been parsed (or intentionally omitted) into
/// `update_stmt` by the caller.
pub(crate) fn parse_update_statement_body(p: &mut SqlParser, update_stmt: Marker) -> ParsedSyntax {
    p.expect(T![update]);
    parse_table_binding(p).or_add_diagnostic(p, expected_table_binding);
    parse_set_clause(p);
    let _ = parse_update_from_clause(p);
    let _ = parse_where_clause(p);
    let _ = parse_returning_clause(p);
    p.eat(T![;]);

    Present(update_stmt.complete(p, SQL_UPDATE_STATEMENT))
}

/// `UPDATE t SET ... FROM other_table WHERE ...` -- brings extra tables
/// into scope for the `SET`/`WHERE` expressions, e.g. to update one table
/// using values joined from another.
fn parse_update_from_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![from]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![from]);
    SqlFromItemList.parse_list(p);
    Present(m.complete(p, SQL_UPDATE_FROM_CLAUSE))
}

pub(crate) fn parse_set_clause(p: &mut SqlParser) -> CompletedMarker {
    let m = p.start();
    p.expect(T![set]);
    SqlSetItemList.parse_list(p);
    m.complete(p, SQL_SET_CLAUSE)
}

struct SqlSetItemList;

impl ParseSeparatedList for SqlSetItemList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_SET_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_set_item(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
            || p.at(T![;])
            || p.at(T![from])
            || p.at(T![where])
            || p.at(T![returning])
            || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SQL_BOGUS_ASSIGNMENT, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_empty(&self) -> bool {
        false
    }
}

fn parse_set_item(p: &mut SqlParser) -> ParsedSyntax {
    if !is_at_name_start(p) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).unwrap();
    p.expect(T![=]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, SQL_SET_ITEM))
}
