use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{EXPR_RECOVERY_SET, parse_expression, parse_name};
use super::from::parse_table_binding;
use super::parse_error::*;
use super::returning_clause::parse_returning_clause;
use super::where_clause::parse_where_clause;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

pub(crate) fn parse_update_statement(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![update]) {
        return Absent;
    }

    let m = p.start();
    parse_update_statement_body(p, m)
}

/// Parses the body of an `update` statement, assuming an optional leading
/// `with` clause has already been parsed (or intentionally omitted) into
/// `update_stmt` by the caller.
pub(crate) fn parse_update_statement_body(p: &mut PsqlParser, update_stmt: Marker) -> ParsedSyntax {
    p.expect(T![update]);
    parse_table_binding(p).or_add_diagnostic(p, expected_table_binding);
    parse_set_clause(p);
    let _ = parse_where_clause(p);
    let _ = parse_returning_clause(p);
    p.eat(T![;]);

    Present(update_stmt.complete(p, PSQL_UPDATE_STATEMENT))
}

fn parse_set_clause(p: &mut PsqlParser) -> CompletedMarker {
    let m = p.start();
    p.expect(T![set]);
    PsqlSetItemList.parse_list(p);
    m.complete(p, PSQL_SET_CLAUSE)
}

struct PsqlSetItemList;

impl ParseSeparatedList for PsqlSetItemList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_SET_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_set_item(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![;]) || p.at(T![where])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_ASSIGNMENT, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_set_item(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).unwrap();
    p.expect(T![=]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_SET_ITEM))
}
