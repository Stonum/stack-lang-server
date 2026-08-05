use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlHavingClause;
use sql_syntax::SqlHavingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlHavingClause;
impl FormatNodeRule<SqlHavingClause> for FormatSqlHavingClause {
    fn fmt_fields(&self, node: &SqlHavingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlHavingClauseFields {
            having_token,
            condition,
        } = node.as_fields();

        write!(f, [having_token.format(), space(), condition.format()])
    }
}
