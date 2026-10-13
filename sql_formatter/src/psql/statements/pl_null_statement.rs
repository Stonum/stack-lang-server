use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlNullStatement;
use sql_syntax::PsqlPlNullStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlNullStatement;
impl FormatNodeRule<PsqlPlNullStatement> for FormatPsqlPlNullStatement {
    fn fmt_fields(&self, node: &PsqlPlNullStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlNullStatementFields {
            null_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [null_token.format(), semicolon_token.format()])
    }
}
