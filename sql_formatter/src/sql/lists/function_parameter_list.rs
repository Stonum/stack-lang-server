use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlFunctionParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFunctionParameterList;
impl FormatRule<SqlFunctionParameterList> for FormatSqlFunctionParameterList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlFunctionParameterList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
