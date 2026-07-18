use crate::prelude::*;
use psql_syntax::PsqlCaseWhenClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCaseWhenClauseList;
impl FormatRule<PsqlCaseWhenClauseList> for FormatPsqlCaseWhenClauseList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlCaseWhenClauseList, f: &mut PsqlFormatter) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
