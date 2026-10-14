use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlSqlBody;
use sql_syntax::PsqlSqlBodyFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSqlBody;
impl FormatNodeRule<PsqlSqlBody> for FormatPsqlSqlBody {
    fn fmt_fields(&self, node: &PsqlSqlBody, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlSqlBodyFields {
            l_quote_token,
            statements,
            r_quote_token,
        } = node.as_fields();

        // Same shape as a PL/pgSQL body: the statements on their own lines
        // between the quotes.
        write!(
            f,
            [
                l_quote_token.format(),
                hard_line_break(),
                statements.format(),
                hard_line_break(),
                r_quote_token.format()
            ]
        )
    }
}
