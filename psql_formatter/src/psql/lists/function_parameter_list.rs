use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlFunctionParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFunctionParameterList;
impl FormatRule<PsqlFunctionParameterList> for FormatPsqlFunctionParameterList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlFunctionParameterList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
