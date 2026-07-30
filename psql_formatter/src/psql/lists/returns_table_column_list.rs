use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlReturnsTableColumnList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturnsTableColumnList;
impl FormatRule<PsqlReturnsTableColumnList> for FormatPsqlReturnsTableColumnList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlReturnsTableColumnList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
