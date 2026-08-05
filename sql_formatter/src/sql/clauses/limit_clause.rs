use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlLimitClause;
use sql_syntax::SqlLimitClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlLimitClause;
impl FormatNodeRule<SqlLimitClause> for FormatSqlLimitClause {
    fn fmt_fields(&self, node: &SqlLimitClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlLimitClauseFields {
            limit_token,
            limit_count,
        } = node.as_fields();

        write!(f, [limit_token.format(), space(), limit_count.format()])
    }
}
