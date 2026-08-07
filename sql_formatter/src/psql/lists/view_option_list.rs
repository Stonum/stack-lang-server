use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::PsqlViewOptionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlViewOptionList;
impl FormatRule<PsqlViewOptionList> for FormatPsqlViewOptionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlViewOptionList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
