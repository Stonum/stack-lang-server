use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlValuesRowList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlValuesRowList;
impl FormatRule<SqlValuesRowList> for FormatSqlValuesRowList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlValuesRowList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
