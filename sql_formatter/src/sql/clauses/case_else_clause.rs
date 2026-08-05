use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCaseElseClause;
use sql_syntax::SqlCaseElseClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCaseElseClause;
impl FormatNodeRule<SqlCaseElseClause> for FormatSqlCaseElseClause {
    fn fmt_fields(&self, node: &SqlCaseElseClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCaseElseClauseFields { else_token, result } = node.as_fields();

        write!(f, [else_token.format(), space(), result.format()])
    }
}
