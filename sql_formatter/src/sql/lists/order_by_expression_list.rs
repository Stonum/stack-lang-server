use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlOrderByExpressionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlOrderByExpressionList;
impl FormatRule<SqlOrderByExpressionList> for FormatSqlOrderByExpressionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlOrderByExpressionList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
