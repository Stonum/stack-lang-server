use crate::prelude::*;
use sql_syntax::PsqlPlStatementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlStatementList;
impl FormatRule<PsqlPlStatementList> for FormatPsqlPlStatementList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlStatementList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(hard_line_break())
            .entries(node.iter().formatted())
            .finish()
    }
}
