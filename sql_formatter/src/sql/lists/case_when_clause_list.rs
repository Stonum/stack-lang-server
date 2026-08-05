use crate::prelude::*;
use sql_syntax::SqlCaseWhenClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCaseWhenClauseList;
impl FormatRule<SqlCaseWhenClauseList> for FormatSqlCaseWhenClauseList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlCaseWhenClauseList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
