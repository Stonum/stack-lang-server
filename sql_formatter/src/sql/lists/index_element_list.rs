use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlIndexElementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlIndexElementList;
impl FormatRule<SqlIndexElementList> for FormatSqlIndexElementList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlIndexElementList, f: &mut SqlFormatter) -> FormatResult<()> {
        // Fallback only: `SqlCreateIndexStatement` uses `write_bracketed_fill_list`.
        write_wrapping_separated_list(node, f)
    }
}
