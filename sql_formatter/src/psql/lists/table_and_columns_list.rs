use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::PsqlTableAndColumnsList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableAndColumnsList;
impl FormatRule<PsqlTableAndColumnsList> for FormatPsqlTableAndColumnsList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlTableAndColumnsList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
