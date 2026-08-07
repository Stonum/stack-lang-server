use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::PsqlFunctionParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFunctionParameterList;
impl FormatRule<PsqlFunctionParameterList> for FormatPsqlFunctionParameterList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlFunctionParameterList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
