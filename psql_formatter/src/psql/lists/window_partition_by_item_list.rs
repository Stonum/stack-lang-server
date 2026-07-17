use crate::prelude::*;
use psql_syntax::PsqlWindowPartitionByItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWindowPartitionByItemList;
impl FormatRule<PsqlWindowPartitionByItemList> for FormatPsqlWindowPartitionByItemList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlWindowPartitionByItemList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
