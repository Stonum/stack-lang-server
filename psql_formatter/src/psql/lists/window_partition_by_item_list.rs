use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlWindowPartitionByItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWindowPartitionByItemList;
impl FormatRule<PsqlWindowPartitionByItemList> for FormatPsqlWindowPartitionByItemList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlWindowPartitionByItemList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
