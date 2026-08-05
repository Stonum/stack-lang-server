use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use sql_syntax::AnySqlExpression;
use sql_syntax::SqlGroupByClause;
use sql_syntax::SqlGroupByClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlGroupByClause;
impl FormatNodeRule<SqlGroupByClause> for FormatSqlGroupByClause {
    fn fmt_fields(&self, node: &SqlGroupByClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlGroupByClauseFields {
            group_by_token,
            items,
        } = node.as_fields();

        write_wrapping_fill_clause(
            group_by_token.format(),
            &items,
            |expr: &AnySqlExpression| !is_simple_expression(expr, 0),
            f,
        )
    }
}
