use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{EXPR_RECOVERY_SET, is_at_name_start, parse_column_name_list, parse_name};
use super::parse_error::*;
use super::postgres::with_clause::{eat_recursive_modifier, parse_cte_materialized_hint};
use super::select::parse_select_statement_body;
use super::stmt::parse_statement;
use super::values::parse_values_clause_body;
use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind::*, T, *};

/// `with [recursive] cte [(cols)] as (query) [, cte2 ...]`, an optional
/// prefix shared by `SELECT`, `INSERT`, `UPDATE` and `DELETE`.
pub(crate) fn parse_with_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![with]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![with]);
    eat_recursive_modifier(p);
    SqlCteDefinitionList.parse_list(p);
    Present(m.complete(p, SQL_WITH_CLAUSE))
}

/// A `select` or `values` body with an optional leading `with` clause,
/// e.g. the contents of a subquery `(...)`. Unlike a top-level statement
/// (which can dispatch to `select`/`values`/`insert`/`update`/`delete`
/// after `with`), a subquery's body can only ever be a `select` or a bare
/// `values (...), (...)` (real Postgres allows a `VALUES` list wherever a
/// derived table is expected, e.g. `FROM (VALUES (1, 2), (3, 4)) AS
/// v(a, b)`) -- but parsing `with` still can't live inside `select.rs`
/// itself, same as the top-level case in `stmt.rs`.
pub(crate) fn parse_with_prefixed_select_statement(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    let _ = parse_with_clause(p);
    if p.at(T![values]) {
        return parse_values_clause_body(p, m);
    }
    parse_select_statement_body(p, m)
}

struct SqlCteDefinitionList;

impl ParseSeparatedList for SqlCteDefinitionList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_CTE_DEFINITION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_cte_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // `is_at_list_end` is checked while still positioned on the `,`
        // separator (before it's consumed), so it must recognize the
        // statement keywords that follow the whole `with` clause rather
        // than "not at an identifier" (which would misfire on `,`).
        p.at(EOF)
            || p.at(T![select])
            || p.at(T![values])
            || p.at(T![insert])
            || p.at(T![update])
            || p.at(T![delete])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SQL_BOGUS, EXPR_RECOVERY_SET),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_cte_definition(p: &mut SqlParser) -> ParsedSyntax {
    if !is_at_name_start(p) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).unwrap();
    let _ = parse_column_name_list(p);
    p.expect(T![as]);
    let _ = parse_cte_materialized_hint(p);
    p.expect(T!['(']);
    // A CTE's body can be any statement, not just `select` -- Postgres
    // allows data-modifying CTEs (`insert`/`update`/`delete ... returning
    // ...`) whose output rows feed into the outer query.
    parse_statement(p).or_add_diagnostic(p, expected_statement);
    p.expect(T![')']);
    Present(m.complete(p, SQL_CTE_DEFINITION))
}
