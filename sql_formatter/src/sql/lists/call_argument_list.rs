use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlCallArgumentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCallArgumentList;
impl FormatRule<SqlCallArgumentList> for FormatSqlCallArgumentList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlCallArgumentList, f: &mut SqlFormatter) -> FormatResult<()> {
        // Fallback only: `SqlCallExpression` formats its arguments with
        // `write_bracketed_fill_list`, which owns the surrounding parens.
        write_wrapping_separated_list(node, f)
    }
}
