use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSubstringFromClause;
use sql_syntax::SqlSubstringFromClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSubstringFromClause;
impl FormatNodeRule<SqlSubstringFromClause> for FormatSqlSubstringFromClause {
    fn fmt_fields(&self, node: &SqlSubstringFromClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSubstringFromClauseFields { from_token, value } = node.as_fields();

        write!(f, [from_token.format(), space(), value.format()])
    }
}
