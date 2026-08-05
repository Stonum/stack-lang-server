use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use sql_syntax::SqlDeleteUsingClause;
use sql_syntax::SqlDeleteUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDeleteUsingClause;
impl FormatNodeRule<SqlDeleteUsingClause> for FormatSqlDeleteUsingClause {
    fn fmt_fields(&self, node: &SqlDeleteUsingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDeleteUsingClauseFields { using_token, items } = node.as_fields();

        write_wrapping_clause(using_token, &items, f)
    }
}
