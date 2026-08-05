use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlFromItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFromItemList;
impl FormatRule<SqlFromItemList> for FormatSqlFromItemList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlFromItemList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
