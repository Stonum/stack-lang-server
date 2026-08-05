use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use sql_syntax::SqlOrderByClause;
use sql_syntax::SqlOrderByClauseFields;
use sql_syntax::SqlOrderByExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlOrderByClause;
impl FormatNodeRule<SqlOrderByClause> for FormatSqlOrderByClause {
    fn fmt_fields(&self, node: &SqlOrderByClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlOrderByClauseFields {
            order_by_token,
            items,
        } = node.as_fields();

        write_wrapping_fill_clause(
            order_by_token.format(),
            &items,
            |item: &SqlOrderByExpression| {
                item.item()
                    .is_ok_and(|expr| !is_simple_expression(&expr, 0))
            },
            f,
        )
    }
}
