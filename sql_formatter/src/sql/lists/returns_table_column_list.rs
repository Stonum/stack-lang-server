use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlReturnsTableColumnList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlReturnsTableColumnList;
impl FormatRule<SqlReturnsTableColumnList> for FormatSqlReturnsTableColumnList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlReturnsTableColumnList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
