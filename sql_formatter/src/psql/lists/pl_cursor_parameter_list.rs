use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::PsqlPlCursorParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlCursorParameterList;
impl FormatRule<PsqlPlCursorParameterList> for FormatPsqlPlCursorParameterList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlCursorParameterList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
