use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlOpenStatement;
use sql_syntax::PsqlPlOpenStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlOpenStatement;
impl FormatNodeRule<PsqlPlOpenStatement> for FormatPsqlPlOpenStatement {
    fn fmt_fields(&self, node: &PsqlPlOpenStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlOpenStatementFields {
            open_token,
            cursor,
            arguments,
            scroll,
            for_clause,
            semicolon_token,
        } = node.as_fields();

        write!(f, [open_token.format(), space(), cursor.format()])?;
        if let Some(arguments) = arguments {
            write!(f, [arguments.format()])?;
        }
        if let Some(scroll) = scroll {
            write!(f, [space(), scroll.format()])?;
        }
        if let Some(for_clause) = for_clause {
            write!(f, [space(), for_clause.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
