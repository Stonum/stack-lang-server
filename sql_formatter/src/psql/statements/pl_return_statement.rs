use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlReturnStatement;
use sql_syntax::PsqlPlReturnStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlReturnStatement;
impl FormatNodeRule<PsqlPlReturnStatement> for FormatPsqlPlReturnStatement {
    fn fmt_fields(&self, node: &PsqlPlReturnStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlReturnStatementFields {
            return_token,
            next_token,
            value,
            semicolon_token,
        } = node.as_fields();

        write!(f, [return_token.format()])?;
        if let Some(next_token) = next_token {
            write!(f, [space(), next_token.format()])?;
        }
        if let Some(value) = value {
            write!(f, [space(), value.format()])?;
        }
        write!(f, [semicolon_token.format()])
    }
}
