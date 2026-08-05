use crate::prelude::*;
use sql_syntax::SqlStatementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlStatementList;
impl FormatRule<SqlStatementList> for FormatSqlStatementList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlStatementList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(hard_line_break())
            .entries(node.iter().formatted())
            .finish()
    }
}
