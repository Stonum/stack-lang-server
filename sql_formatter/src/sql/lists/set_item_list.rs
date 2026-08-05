use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlSetItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSetItemList;
impl FormatRule<SqlSetItemList> for FormatSqlSetItemList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlSetItemList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
