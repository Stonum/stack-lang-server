use crate::prelude::*;
use sql_syntax::PsqlCopyOptionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCopyOptionList;
impl FormatRule<PsqlCopyOptionList> for FormatPsqlCopyOptionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlCopyOptionList, f: &mut SqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
