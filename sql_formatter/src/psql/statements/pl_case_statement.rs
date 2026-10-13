use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlCaseStatement;
use sql_syntax::PsqlPlCaseStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlCaseStatement;
impl FormatNodeRule<PsqlPlCaseStatement> for FormatPsqlPlCaseStatement {
    fn fmt_fields(&self, node: &PsqlPlCaseStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlCaseStatementFields {
            case_token,
            value,
            when_clauses,
            else_clause,
            end_token,
            end_case_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [case_token.format()])?;
        if let Some(value) = value {
            write!(f, [space(), value.format()])?;
        }
        write!(
            f,
            [block_indent(&format_with(|f| {
                write!(f, [when_clauses.format()])?;
                if let Some(else_clause) = &else_clause {
                    write!(f, [else_clause.format()])?;
                }
                Ok(())
            }))]
        )?;
        write!(
            f,
            [
                end_token.format(),
                space(),
                end_case_token.format(),
                semicolon_token.format()
            ]
        )
    }
}
