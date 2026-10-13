use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlCloseStatement;
use sql_syntax::PsqlPlCloseStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlCloseStatement;
impl FormatNodeRule<PsqlPlCloseStatement> for FormatPsqlPlCloseStatement {
    fn fmt_fields(&self, node: &PsqlPlCloseStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlCloseStatementFields {
            close_token,
            cursor,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                close_token.format(),
                space(),
                cursor.format(),
                semicolon_token.format()
            ]
        )
    }
}
