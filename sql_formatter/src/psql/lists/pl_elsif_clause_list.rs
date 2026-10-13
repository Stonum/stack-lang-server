use crate::prelude::*;
use sql_syntax::PsqlPlElsifClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlElsifClauseList;
impl FormatRule<PsqlPlElsifClauseList> for FormatPsqlPlElsifClauseList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlElsifClauseList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
