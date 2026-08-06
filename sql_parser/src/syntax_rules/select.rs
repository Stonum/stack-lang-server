use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{
    EXPR_RECOVERY_SET, is_at_table_star, parse_alias, parse_expression, parse_limit_offset_value,
    parse_table_star,
};
use super::from::parse_from_clause;
use super::parse_error::*;
use super::postgres::select::{parse_distinct_on_clause, parse_limit_clause};
use super::where_clause::parse_where_clause;
use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind::*, T, *};

pub(crate) fn parse_select_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![select]) {
        return Absent;
    }

    let select_stmt = p.start();
    parse_select_statement_body(p, select_stmt)
}

/// Parses the body of a `select` statement, assuming an optional leading
/// `with` clause has already been parsed (or intentionally omitted) into
/// `select_stmt` by the caller.
pub(crate) fn parse_select_statement_body(p: &mut SqlParser, select_stmt: Marker) -> ParsedSyntax {
    parse_select_core(p);
    parse_set_operation_list(p);
    let _ = parse_order_by_clause(p);
    let _ = parse_limit_clause(p);
    let _ = parse_offset_clause(p);
    let _ = parse_fetch_clause(p);
    p.eat(T![;]);

    Present(select_stmt.complete(p, SQL_SELECT_STATEMENT))
}

/// `select ...` plus its `from`/`where`/`group by`/`having` clauses, i.e.
/// everything that can appear on either side of a `union`/`intersect`/
/// `except`. Shared between the leading branch (parsed directly into
/// `select_stmt`) and every subsequent [SqlSetOperation] branch.
fn parse_select_core(p: &mut SqlParser) {
    let select_clause = p.start();
    p.expect(T![select]);
    let _ = parse_select_quantifier(p);
    SqlSelectItemList.parse_list(p);
    select_clause.complete(p, SQL_SELECT_CLAUSE);

    let _ = parse_from_clause(p);
    let _ = parse_where_clause(p);
    let _ = parse_group_by_clause(p);
    let _ = parse_having_clause(p);
}

/// `all` or `distinct [on (expr, ...)]` right after `select`.
fn parse_select_quantifier(p: &mut SqlParser) -> ParsedSyntax {
    if p.at(T![all]) {
        let m = p.start();
        p.bump(T![all]);
        Present(m.complete(p, SQL_SELECT_ALL_QUANTIFIER))
    } else if p.at(T![distinct]) {
        let m = p.start();
        p.bump(T![distinct]);
        let _ = parse_distinct_on_clause(p);
        Present(m.complete(p, SQL_SELECT_DISTINCT_QUANTIFIER))
    } else {
        Absent
    }
}

/// Zero or more `union`/`intersect`/`except` branches following the leading
/// `select`. `order by`/`limit`/`offset` apply to the combined result of the
/// whole chain, so they live outside this list, on `SqlSelectStatement`
/// itself, rather than on each branch.
fn parse_set_operation_list(p: &mut SqlParser) -> CompletedMarker {
    let m = p.start();
    while is_at_set_operator(p) {
        let _ = parse_set_operation(p);
    }
    m.complete(p, SQL_SET_OPERATION_LIST)
}

fn is_at_set_operator(p: &mut SqlParser) -> bool {
    p.at(T![union]) || p.at(T![intersect]) || p.at(T![except])
}

fn parse_set_operation(p: &mut SqlParser) -> ParsedSyntax {
    if !is_at_set_operator(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_any();
    if p.at(T![all]) || p.at(T![distinct]) {
        p.bump_any();
    }
    parse_select_core(p);
    Present(m.complete(p, SQL_SET_OPERATION))
}

fn parse_group_by_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![group_by]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![group_by]);
    SqlGroupByItemList.parse_list(p);
    Present(m.complete(p, SQL_GROUP_BY_CLAUSE))
}

struct SqlGroupByItemList;

impl ParseSeparatedList for SqlGroupByItemList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_GROUP_BY_ITEM_LIST;

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
            || p.at(T![fetch])
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
            &ParseRecoveryTokenSet::new(SQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_empty(&self) -> bool {
        false
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_having_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![having]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![having]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, SQL_HAVING_CLAUSE))
}

pub(crate) fn parse_order_by_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![order_by]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![order_by]);
    SqlOrderByExpressionList.parse_list(p);
    Present(m.complete(p, SQL_ORDER_BY_CLAUSE))
}

struct SqlOrderByExpressionList;

impl ParseSeparatedList for SqlOrderByExpressionList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_ORDER_BY_EXPRESSION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_order_by_expression(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
            || p.at(T![;])
            || p.at(T![limit])
            || p.at(T![offset])
            || p.at(T![fetch])
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
            &ParseRecoveryTokenSet::new(SQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
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

fn parse_order_by_expression(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    if parse_expression(p).is_present() {
        if p.at(T![asc]) || p.at(T![desc]) {
            p.bump_any();
        }
        Present(m.complete(p, SQL_ORDER_BY_EXPRESSION))
    } else {
        m.abandon(p);
        Absent
    }
}

fn parse_offset_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![offset]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![offset]);
    parse_limit_offset_value(p).or_add_diagnostic(p, expected_limit_value);
    Present(m.complete(p, SQL_OFFSET_CLAUSE))
}

/// SQL-standard `FETCH { FIRST | NEXT } [count] { ROW | ROWS } { ONLY |
/// WITH TIES }` -- an alternative spelling of `LIMIT`, real Postgres pairs
/// it with `OFFSET`.
fn parse_fetch_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![fetch]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![fetch]);
    if p.at(T![first]) || p.at(T![next]) {
        p.bump_any();
    } else {
        let range = p.cur_range();
        let err = p
            .err_builder("Expected `first` or `next` after `fetch`", range)
            .with_hint("Only `FETCH FIRST`/`FETCH NEXT` is supported");
        p.error(err);
    }
    let _ = parse_limit_offset_value(p);
    if p.at(T![row]) || p.at(T![rows]) {
        p.bump_any();
    } else {
        let range = p.cur_range();
        let err = p
            .err_builder("Expected `row` or `rows`", range)
            .with_hint("The fetch count must be followed by `ROW` or `ROWS`");
        p.error(err);
    }
    parse_fetch_tail(p).or_add_diagnostic(p, expected_fetch_tail);
    Present(m.complete(p, SQL_FETCH_CLAUSE))
}

fn parse_fetch_tail(p: &mut SqlParser) -> ParsedSyntax {
    if p.at(T![only]) {
        let m = p.start();
        p.bump(T![only]);
        return Present(m.complete(p, SQL_FETCH_ONLY_TAIL));
    }

    if p.at(T![with]) {
        let m = p.start();
        p.bump(T![with]);
        p.expect(T![ties]);
        return Present(m.complete(p, SQL_FETCH_WITH_TIES_TAIL));
    }

    Absent
}

struct SqlSelectItemList;

impl ParseSeparatedList for SqlSelectItemList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_SELECT_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_select_item(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
            || p.at(T![;])
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
            || p.at(T![fetch])
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
            &ParseRecoveryTokenSet::new(SQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

pub(crate) fn parse_select_item(p: &mut SqlParser) -> ParsedSyntax {
    if p.at(T![*]) {
        let m = p.start();
        p.bump(T![*]);
        return Present(m.complete(p, SQL_STAR));
    }
    if is_at_table_star(p) {
        return parse_table_star(p);
    }

    let m = p.start();
    if parse_expression(p).is_present() {
        parse_alias(p);
        Present(m.complete(p, SQL_SELECT_EXPRESSION))
    } else {
        m.abandon(p);
        Absent
    }
}
