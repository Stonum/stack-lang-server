use crate::prelude::*;
use sql_syntax::SqlFunctionOptionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFunctionOptionList;
impl FormatRule<SqlFunctionOptionList> for FormatSqlFunctionOptionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlFunctionOptionList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
