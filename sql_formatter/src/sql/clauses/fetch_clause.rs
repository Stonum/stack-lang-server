use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlFetchClause;
use sql_syntax::SqlFetchClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFetchClause;
impl FormatNodeRule<SqlFetchClause> for FormatSqlFetchClause {
    fn fmt_fields(&self, node: &SqlFetchClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlFetchClauseFields {
            fetch_token,
            quantifier,
            count,
            row_or_rows,
            tail,
        } = node.as_fields();

        write!(f, [fetch_token.format(), space(), quantifier.format()])?;
        if let Some(count) = count {
            write!(f, [space(), count.format()])?;
        }
        write!(f, [space(), row_or_rows.format(), space(), tail.format(),])
    }
}
