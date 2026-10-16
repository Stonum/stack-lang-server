use crate::embedded_sql::FormatExecuteCommand;
use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlExecuteStatement;
use sql_syntax::PsqlPlExecuteStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlExecuteStatement;
impl FormatNodeRule<PsqlPlExecuteStatement> for FormatPsqlPlExecuteStatement {
    fn fmt_fields(&self, node: &PsqlPlExecuteStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlExecuteStatementFields {
            execute_token,
            command,
            into_clause,
            using_clause,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_once(|f| {
                write!(
                    f,
                    [
                        execute_token.format(),
                        space(),
                        FormatExecuteCommand(&command)
                    ]
                )?;
                if let Some(into_clause) = into_clause {
                    write!(f, [soft_line_break_or_space(), into_clause.format()])?;
                }
                if let Some(using_clause) = using_clause {
                    write!(f, [soft_line_break_or_space(), using_clause.format()])?;
                }
                Ok(())
            }))]
        )?;
        write!(f, [semicolon_token.format()])
    }
}
