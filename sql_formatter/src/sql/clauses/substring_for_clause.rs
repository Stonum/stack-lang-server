use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSubstringForClause;
use sql_syntax::SqlSubstringForClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSubstringForClause;
impl FormatNodeRule<SqlSubstringForClause> for FormatSqlSubstringForClause {
    fn fmt_fields(&self, node: &SqlSubstringForClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSubstringForClauseFields { for_token, value } = node.as_fields();

        write!(f, [for_token.format(), space(), value.format()])
    }
}
