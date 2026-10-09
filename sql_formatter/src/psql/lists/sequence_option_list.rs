use crate::prelude::*;
use sql_syntax::PsqlSequenceOptionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSequenceOptionList;
impl FormatRule<PsqlSequenceOptionList> for FormatPsqlSequenceOptionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlSequenceOptionList, f: &mut SqlFormatter) -> FormatResult<()> {
        let mut filler = f.fill();
        for option in node.iter() {
            filler.entry(&soft_line_break_or_space(), &option.format());
        }
        filler.finish()
    }
}
