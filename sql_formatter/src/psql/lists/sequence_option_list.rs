use crate::prelude::*;
use sql_syntax::PsqlSequenceOptionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSequenceOptionList;
impl FormatRule<PsqlSequenceOptionList> for FormatPsqlSequenceOptionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlSequenceOptionList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(space())
            .entries(node.iter().formatted())
            .finish()
    }
}
