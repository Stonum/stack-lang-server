use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlWindowPartitionByItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWindowPartitionByItemList;
impl FormatRule<SqlWindowPartitionByItemList> for FormatSqlWindowPartitionByItemList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlWindowPartitionByItemList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
