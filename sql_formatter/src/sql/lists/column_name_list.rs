use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlColumnNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlColumnNameList;
impl FormatRule<SqlColumnNameList> for FormatSqlColumnNameList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlColumnNameList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
