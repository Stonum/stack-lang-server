use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlExitStatement;
use sql_syntax::PsqlPlExitStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlExitStatement;
impl FormatNodeRule<PsqlPlExitStatement> for FormatPsqlPlExitStatement {
    fn fmt_fields(&self, node: &PsqlPlExitStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlExitStatementFields {
            keyword,
            label,
            when_clause,
            semicolon_token,
        } = node.as_fields();

        write!(f, [keyword.format()])?;
        if let Some(label) = label {
            write!(f, [space(), label.format()])?;
        }
        if let Some(when_clause) = when_clause {
            write!(f, [space(), when_clause.format()])?;
        }
        write!(f, [semicolon_token.format()])
    }
}
