use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlReturnsTableClause;
use psql_syntax::PsqlReturnsTableClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturnsTableClause;
impl FormatNodeRule<PsqlReturnsTableClause> for FormatPsqlReturnsTableClause {
    fn fmt_fields(&self, node: &PsqlReturnsTableClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlReturnsTableClauseFields {
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
