use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use sql_syntax::SqlFromClause;
use sql_syntax::SqlFromClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFromClause;
impl FormatNodeRule<SqlFromClause> for FormatSqlFromClause {
    fn fmt_fields(&self, node: &SqlFromClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlFromClauseFields { from_token, items } = node.as_fields();

        write_wrapping_clause(from_token, &items, f)
    }
}
