use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlFetchStatement;
use sql_syntax::PsqlPlFetchStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlFetchStatement;
impl FormatNodeRule<PsqlPlFetchStatement> for FormatPsqlPlFetchStatement {
    fn fmt_fields(&self, node: &PsqlPlFetchStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlFetchStatementFields {
            keyword,
            direction,
            from_token,
            cursor,
            into_clause,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_once(|f| {
                write!(f, [keyword.format()])?;
                if let Some(direction) = direction {
                    write!(f, [space(), direction.format()])?;
                }
                if let Some(from_token) = from_token {
                    write!(f, [space(), from_token.format()])?;
                }
                write!(f, [space(), cursor.format()])?;
                if let Some(into_clause) = into_clause {
                    write!(f, [soft_line_break_or_space(), into_clause.format()])?;
                }
                Ok(())
            }))]
        )?;
        write!(f, [semicolon_token.format()])
    }
}
