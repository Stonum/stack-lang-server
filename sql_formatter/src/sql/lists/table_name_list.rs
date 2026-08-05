use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlTableNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTableNameList;
impl FormatRule<SqlTableNameList> for FormatSqlTableNameList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlTableNameList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
