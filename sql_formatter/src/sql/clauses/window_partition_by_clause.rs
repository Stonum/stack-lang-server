use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use sql_syntax::AnySqlExpression;
use sql_syntax::SqlWindowPartitionByClause;
use sql_syntax::SqlWindowPartitionByClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWindowPartitionByClause;
impl FormatNodeRule<SqlWindowPartitionByClause> for FormatSqlWindowPartitionByClause {
    fn fmt_fields(
        &self,
        node: &SqlWindowPartitionByClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlWindowPartitionByClauseFields {
            partition_by_token,
            items,
        } = node.as_fields();

        write_wrapping_fill_clause(
            partition_by_token.format(),
            &items,
            |expr: &AnySqlExpression| !is_simple_expression(expr, 0),
            f,
        )
    }
}
