use crate::prelude::*;
use sql_syntax::PsqlPlExceptionHandlerList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlExceptionHandlerList;
impl FormatRule<PsqlPlExceptionHandlerList> for FormatPsqlPlExceptionHandlerList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlExceptionHandlerList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
