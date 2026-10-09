use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlIndexIncludeClause;
use sql_syntax::SqlIndexIncludeClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlIndexIncludeClause;
impl FormatNodeRule<SqlIndexIncludeClause> for FormatSqlIndexIncludeClause {
    fn fmt_fields(&self, node: &SqlIndexIncludeClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlIndexIncludeClauseFields {
            include_token,
            columns,
        } = node.as_fields();

        write!(f, [include_token.format(), space(), columns.format()])
    }
}
