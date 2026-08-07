use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::PsqlReturnsTableColumnList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturnsTableColumnList;
impl FormatRule<PsqlReturnsTableColumnList> for FormatPsqlReturnsTableColumnList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlReturnsTableColumnList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
