use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlSelectItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSelectItemList;
impl FormatRule<SqlSelectItemList> for FormatSqlSelectItemList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlSelectItemList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
