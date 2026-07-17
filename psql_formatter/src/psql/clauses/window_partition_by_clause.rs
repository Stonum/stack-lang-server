use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlWindowPartitionByClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWindowPartitionByClause;
impl FormatNodeRule<PsqlWindowPartitionByClause> for FormatPsqlWindowPartitionByClause {
    fn fmt_fields(
        &self,
        node: &PsqlWindowPartitionByClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
