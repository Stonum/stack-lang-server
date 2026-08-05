use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlWhereClause;
use sql_syntax::SqlWhereClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWhereClause;
impl FormatNodeRule<SqlWhereClause> for FormatSqlWhereClause {
    fn fmt_fields(&self, node: &SqlWhereClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlWhereClauseFields {
            where_token,
            condition,
        } = node.as_fields();

        write!(f, [where_token.format(), space(), condition.format()])
    }
}
