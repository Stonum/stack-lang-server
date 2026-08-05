use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlExpressionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlExpressionList;
impl FormatRule<SqlExpressionList> for FormatSqlExpressionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlExpressionList, f: &mut SqlFormatter) -> FormatResult<()> {
        // This is the plain fallback used if something ever calls `.format()`
        // on this list directly. The bracket-owning call sites (call
        // arguments, INSERT...VALUES, array literals) bypass this and call
        // `write_bracketed_fill_list` instead, since the fill/flat decision
        // needs to own the surrounding parens -- see that function's doc
        // comment in `sql_formatter/src/utils.rs`.
        write_wrapping_separated_list(node, f)
    }
}
