use crate::prelude::*;
use crate::utils::write_wrapping_clause;
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

        write_wrapping_clause(partition_by_token, &items, f)
    }
}
