use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{
    EXPR_RECOVERY_SET, parse_alias, parse_expression, parse_number_literal_expression,
};
use super::from::parse_from_clause;
use super::parse_error::*;
use super::where_clause::parse_where_clause;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

pub(crate) fn parse_select_statement(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![select]) {
        return Absent;
    }

    let select_stmt = p.start();
    parse_select_statement_body(p, select_stmt)
}

/// Parses the body of a `select` statement, assuming an optional leading
/// `with` clause has already been parsed (or intentionally omitted) into
/// `select_stmt` by the caller.
pub(crate) fn parse_select_statement_body(p: &mut PsqlParser, select_stmt: Marker) -> ParsedSyntax {
    parse_select_core(p);
    parse_set_operation_list(p);
    let _ = parse_order_by_clause(p);
    let _ = parse_limit_clause(p);
    let _ = parse_offset_clause(p);
    p.eat(T![;]);

    Present(select_stmt.complete(p, PSQL_SELECT_STATEMENT))
}

/// `select ...` plus its `from`/`where`/`group by`/`having` clauses, i.e.
/// everything that can appear on either side of a `union`/`intersect`/
/// `except`. Shared between the leading branch (parsed directly into
/// `select_stmt`) and every subsequent [PsqlSetOperation] branch.
fn parse_select_core(p: &mut PsqlParser) {
    let select_clause = p.start();
    p.expect(T![select]);
    PsqlSelectItemList.parse_list(p);
    select_clause.complete(p, PSQL_SELECT_CLAUSE);

    let _ = parse_from_clause(p);
    let _ = parse_where_clause(p);
    let _ = parse_group_by_clause(p);
    let _ = parse_having_clause(p);
}

/// Zero or more `union`/`intersect`/`except` branches following the leading
/// `select`. `order by`/`limit`/`offset` apply to the combined result of the
/// whole chain, so they live outside this list, on `PsqlSelectStatement`
/// itself, rather than on each branch.
fn parse_set_operation_list(p: &mut PsqlParser) -> CompletedMarker {
    let m = p.start();
    while is_at_set_operator(p) {
        let _ = parse_set_operation(p);
    }
    m.complete(p, PSQL_SET_OPERATION_LIST)
}

fn is_at_set_operator(p: &mut PsqlParser) -> bool {
    p.at(T![union]) || p.at(T![intersect]) || p.at(T![except])
}

fn parse_set_operation(p: &mut PsqlParser) -> ParsedSyntax {
    if !is_at_set_operator(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_any();
    if p.at(T![all]) || p.at(T![distinct]) {
        p.bump_any();
    }
    parse_select_core(p);
    Present(m.complete(p, PSQL_SET_OPERATION))
}

fn parse_group_by_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![group_by]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![group_by]);
    PsqlGroupByItemList.parse_list(p);
    Present(m.complete(p, PSQL_GROUP_BY_CLAUSE))
}

struct PsqlGroupByItemList;

impl ParseSeparatedList for PsqlGroupByItemList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_GROUP_BY_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_expression(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
            || p.at(T![;])
            || p.at(T![having])
            || p.at(T![union])
            || p.at(T![intersect])
            || p.at(T![except])
            || p.at(T![order_by])
            || p.at(T![limit])
            || p.at(T![offset])
            || p.at(T![returning])
            // `insert into t select ... group by ... on conflict ...`
            || p.at(T![on])
            || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_having_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![having]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![having]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_HAVING_CLAUSE))
}

pub(crate) fn parse_order_by_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![order_by]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![order_by]);
    PsqlOrderByExpressionList.parse_list(p);
    Present(m.complete(p, PSQL_ORDER_BY_CLAUSE))
}

struct PsqlOrderByExpressionList;

impl ParseSeparatedList for PsqlOrderByExpressionList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_ORDER_BY_EXPRESSION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_order_by_expression(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
            || p.at(T![;])
            || p.at(T![limit])
            || p.at(T![offset])
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
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_order_by_expression(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    if parse_expression(p).is_present() {
        if p.at(T![asc]) || p.at(T![desc]) {
            p.bump_any();
        }
        Present(m.complete(p, PSQL_ORDER_BY_EXPRESSION))
    } else {
        m.abandon(p);
        Absent
    }
}

fn parse_limit_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![limit]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![limit]);
    parse_number_literal_expression(p).or_add_diagnostic(p, expected_number_literal);
    Present(m.complete(p, PSQL_LIMIT_CLAUSE))
}

fn parse_offset_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![offset]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![offset]);
    parse_number_literal_expression(p).or_add_diagnostic(p, expected_number_literal);
    Present(m.complete(p, PSQL_OFFSET_CLAUSE))
}

struct PsqlSelectItemList;

impl ParseSeparatedList for PsqlSelectItemList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_SELECT_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_select_item(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
            || p.at(T![from])
            || p.at(T![where])
            || p.at(T![group_by])
            || p.at(T![having])
            || p.at(T![union])
            || p.at(T![intersect])
            || p.at(T![except])
            || p.at(T![order_by])
            || p.at(T![limit])
            || p.at(T![offset])
            || p.at(T![returning])
            // `insert into t select 1 on conflict ...` (no `from` clause)
            || p.at(T![on])
            || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

pub(crate) fn parse_select_item(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    if p.at(T![*]) {
        p.bump(T![*]);
        Present(m.complete(p, PSQL_STAR))
    } else if parse_expression(p).is_present() {
        parse_alias(p);
        Present(m.complete(p, PSQL_SELECT_EXPRESSION))
    } else {
        m.abandon(p);
        Absent
    }
}
