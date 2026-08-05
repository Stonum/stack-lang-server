use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlReturnsClause;
use sql_syntax::SqlReturnsClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlReturnsClause;
impl FormatNodeRule<SqlReturnsClause> for FormatSqlReturnsClause {
    fn fmt_fields(&self, node: &SqlReturnsClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlReturnsClauseFields { returns_token, ty } = node.as_fields();

        write!(f, [returns_token.format(), space(), ty.format()])
    }
}
