use crate::prelude::*;
use sql_syntax::PsqlPlCaseWhenClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlCaseWhenClauseList;
impl FormatRule<PsqlPlCaseWhenClauseList> for FormatPsqlPlCaseWhenClauseList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlCaseWhenClauseList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
