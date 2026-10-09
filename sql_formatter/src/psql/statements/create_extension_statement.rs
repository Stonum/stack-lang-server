use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCreateExtensionStatement;
use sql_syntax::PsqlCreateExtensionStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCreateExtensionStatement;
impl FormatNodeRule<PsqlCreateExtensionStatement> for FormatPsqlCreateExtensionStatement {
    fn fmt_fields(
        &self,
        node: &PsqlCreateExtensionStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlCreateExtensionStatementFields {
            create_token,
            extension_token,
            if_token,
            not_token,
            exists_token,
            name,
            schema_clause,
            cascade_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_with(|f: &mut SqlFormatter| {
                write!(
                    f,
                    [create_token.format(), space(), extension_token.format()]
                )?;
                for token in [&if_token, &not_token, &exists_token].into_iter().flatten() {
                    write!(f, [space(), token.format()])?;
                }
                write!(f, [space(), name.format()])?;
                if let Some(schema_clause) = &schema_clause {
                    write!(f, [soft_line_break_or_space(), schema_clause.format()])?;
                }
                if let Some(cascade_token) = &cascade_token {
                    write!(f, [soft_line_break_or_space(), cascade_token.format()])?;
                }
                Ok(())
            }))]
        )?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
