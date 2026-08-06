use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::expr::EXPR_RECOVERY_SET;
use crate::syntax_rules::parse_error::expected_expression;
use crate::syntax_rules::select::parse_select_item;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T, *};

/// `returning *` / `returning col, col as alias, ...`, shared by `INSERT`,
/// `UPDATE` and `DELETE`. Reuses [parse_select_item] for the individual
/// `* | expr [as alias]` elements, but with its own list boundary — unlike
/// a `select` list, a `returning` list is always the last clause before the
/// statement ends. Postgres-only (T-SQL uses `OUTPUT` instead, an unrelated
/// clause shape) -- the `returning` keyword check stays outside the
/// dialect gate so a caller composing this with adjacent optional clauses
/// (`let _ = parse_returning_clause(p);`) still sees a clean `Absent` when
/// there's no `returning` at all, only committing to (and diagnosing) the
/// Postgres-only body once the keyword is actually there.
pub(crate) fn parse_returning_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![returning]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_returning_clause_body,
        |p, marker| postgres_only_syntax_error(p, "`RETURNING` clauses", marker.range(p)),
    )
}

fn parse_returning_clause_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![returning]);
    SqlReturningItemList.parse_list(p);
    Present(m.complete(p, SQL_RETURNING_CLAUSE))
}

struct SqlReturningItemList;

impl ParseSeparatedList for SqlReturningItemList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_SELECT_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_select_item(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // A `returning` clause used inside a data-modifying CTE
        // (`with cte as (insert ... returning id) ...`) is followed by the
        // CTE's closing `)` rather than `;`/EOF, so both must be handled.
        p.at(EOF) || p.at(T![;]) || p.at(T![')'])
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
