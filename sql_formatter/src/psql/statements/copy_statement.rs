use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCopyStatement;
use sql_syntax::PsqlCopyStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCopyStatement;
impl FormatNodeRule<PsqlCopyStatement> for FormatPsqlCopyStatement {
    fn fmt_fields(&self, node: &PsqlCopyStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlCopyStatementFields {
            copy_token,
            source,
            direction,
            target,
            options,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                copy_token.format(),
                space(),
                source.format(),
                space(),
                direction.format(),
                space(),
                target.format()
            ]
        )?;
        if let Some(options) = options {
            write!(f, [space(), options.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
