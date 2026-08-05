use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlReturnsTableClause;
use sql_syntax::SqlReturnsTableClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlReturnsTableClause;
impl FormatNodeRule<SqlReturnsTableClause> for FormatSqlReturnsTableClause {
    fn fmt_fields(&self, node: &SqlReturnsTableClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlReturnsTableClauseFields {
            table_token,
            l_paren_token,
            columns,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                table_token.format(),
                l_paren_token.format(),
                group(&soft_block_indent(&columns.format())),
                r_paren_token.format(),
            ]
        )
    }
}
