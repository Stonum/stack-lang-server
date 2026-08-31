use crate::prelude::*;
use crate::utils::{is_complex_from_item, write_wrapping_fill_clause};
use sql_syntax::SqlUpdateFromClause;
use sql_syntax::SqlUpdateFromClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlUpdateFromClause;
impl FormatNodeRule<SqlUpdateFromClause> for FormatSqlUpdateFromClause {
    fn fmt_fields(&self, node: &SqlUpdateFromClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlUpdateFromClauseFields { from_token, items } = node.as_fields();

        write_wrapping_fill_clause(from_token.format(), &items, is_complex_from_item, f)
    }
}
