use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::EXPR_RECOVERY_SET;
use super::parse_error::*;
use super::select::parse_select_item;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

/// `returning *` / `returning col, col as alias, ...`, shared by `INSERT`,
/// `UPDATE` and `DELETE`. Reuses [parse_select_item] for the individual
/// `* | expr [as alias]` elements, but with its own list boundary — unlike
/// a `select` list, a `returning` list is always the last clause before the
/// statement ends.
pub(crate) fn parse_returning_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![returning]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![returning]);
    PsqlReturningItemList.parse_list(p);
    Present(m.complete(p, PSQL_RETURNING_CLAUSE))
}

struct PsqlReturningItemList;

impl ParseSeparatedList for PsqlReturningItemList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_SELECT_ITEM_LIST;

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
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}
