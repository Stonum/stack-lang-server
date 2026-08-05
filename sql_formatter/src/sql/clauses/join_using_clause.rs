use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlJoinUsingClause;
use sql_syntax::SqlJoinUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlJoinUsingClause;
impl FormatNodeRule<SqlJoinUsingClause> for FormatSqlJoinUsingClause {
    fn fmt_fields(&self, node: &SqlJoinUsingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlJoinUsingClauseFields {
            using_token,
            columns,
        } = node.as_fields();

        write!(f, [using_token.format(), space(), columns.format()])
    }
}
