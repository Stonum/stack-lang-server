use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlDoStatement;
use sql_syntax::PsqlDoStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDoStatement;
impl FormatNodeRule<PsqlDoStatement> for FormatPsqlDoStatement {
    fn fmt_fields(&self, node: &PsqlDoStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlDoStatementFields {
            do_token,
            leading_language,
            body,
            trailing_language,
            semicolon_token,
        } = node.as_fields();

        write!(f, [do_token.format()])?;
        if let Some(leading_language) = leading_language {
            write!(f, [space(), leading_language.format()])?;
        }
        write!(f, [space(), body.format()])?;
        if let Some(trailing_language) = trailing_language {
            write!(f, [space(), trailing_language.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
