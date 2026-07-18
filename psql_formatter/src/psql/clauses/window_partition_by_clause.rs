use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use psql_syntax::PsqlWindowPartitionByClause;
use psql_syntax::PsqlWindowPartitionByClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWindowPartitionByClause;
impl FormatNodeRule<PsqlWindowPartitionByClause> for FormatPsqlWindowPartitionByClause {
    fn fmt_fields(
        &self,
        node: &PsqlWindowPartitionByClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlWindowPartitionByClauseFields {
            partition_by_token,
            items,
        } = node.as_fields();

        write_wrapping_clause(partition_by_token, &items, f)
    }
}
