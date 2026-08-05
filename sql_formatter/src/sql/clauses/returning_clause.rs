use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use sql_syntax::SqlReturningClause;
use sql_syntax::SqlReturningClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlReturningClause;
impl FormatNodeRule<SqlReturningClause> for FormatSqlReturningClause {
    fn fmt_fields(&self, node: &SqlReturningClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlReturningClauseFields {
            returning_token,
            items,
        } = node.as_fields();

        write_wrapping_clause(returning_token, &items, f)
    }
}
