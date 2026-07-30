use crate::prelude::*;
use psql_syntax::PsqlFunctionOptionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFunctionOptionList;
impl FormatRule<PsqlFunctionOptionList> for FormatPsqlFunctionOptionList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlFunctionOptionList, f: &mut PsqlFormatter) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
