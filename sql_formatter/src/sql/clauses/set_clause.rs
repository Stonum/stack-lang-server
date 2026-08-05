use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use sql_syntax::SqlSetClause;
use sql_syntax::SqlSetClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSetClause;
impl FormatNodeRule<SqlSetClause> for FormatSqlSetClause {
    fn fmt_fields(&self, node: &SqlSetClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSetClauseFields { set_token, items } = node.as_fields();

        write_wrapping_clause(set_token, &items, f)
    }
}
