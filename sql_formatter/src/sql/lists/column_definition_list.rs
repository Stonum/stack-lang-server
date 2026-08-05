use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlColumnDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlColumnDefinitionList;
impl FormatRule<SqlColumnDefinitionList> for FormatSqlColumnDefinitionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlColumnDefinitionList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
