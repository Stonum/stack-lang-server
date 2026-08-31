use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use sql_syntax::AnySqlSelectItem;
use sql_syntax::PsqlReturningClause;
use sql_syntax::PsqlReturningClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturningClause;
impl FormatNodeRule<PsqlReturningClause> for FormatPsqlReturningClause {
    fn fmt_fields(&self, node: &PsqlReturningClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlReturningClauseFields {
            returning_token,
            items,
        } = node.as_fields();

        write_wrapping_fill_clause(
            returning_token.format(),
            &items,
            // Same list type and complexity rule as `select`'s own list
            // (`select_clause.rs`) -- `RETURNING` reuses `SqlSelectItemList`.
            |item: &AnySqlSelectItem| match item {
                AnySqlSelectItem::SqlSelectExpression(item) => item
                    .expr()
                    .is_ok_and(|expr| !is_simple_expression(&expr, 0)),
                AnySqlSelectItem::SqlStar(_) => false,
                AnySqlSelectItem::SqlTableStar(_) => false,
            },
            f,
        )
    }
}
