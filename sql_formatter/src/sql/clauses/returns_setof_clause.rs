use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlReturnsSetofClause;
use sql_syntax::SqlReturnsSetofClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlReturnsSetofClause;
impl FormatNodeRule<SqlReturnsSetofClause> for FormatSqlReturnsSetofClause {
    fn fmt_fields(&self, node: &SqlReturnsSetofClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlReturnsSetofClauseFields { setof_token, ty } = node.as_fields();

        write!(f, [setof_token.format(), space(), ty.format()])
    }
}
