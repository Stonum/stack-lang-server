use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlTableElementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTableElementList;
impl FormatRule<SqlTableElementList> for FormatSqlTableElementList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlTableElementList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
