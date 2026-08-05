use crate::prelude::*;
use biome_formatter::{format_args, write};
use sql_syntax::SqlCaseExpression;
use sql_syntax::SqlCaseExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCaseExpression;
impl FormatNodeRule<SqlCaseExpression> for FormatSqlCaseExpression {
    fn fmt_fields(&self, node: &SqlCaseExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCaseExpressionFields {
            case_token,
            expression,
            when_clauses,
            else_clause,
            end_token,
        } = node.as_fields();

        write!(f, [case_token.format()])?;
        if let Some(expression) = expression {
            write!(f, [space(), expression.format()])?;
        }

        // Like a simple SELECT (and unlike a JOIN), collapses onto one line
        // when it fits; only wraps one WHEN per line once it doesn't.
        write!(
            f,
            [group(&format_args![
                soft_space_or_block_indent(&format_once(|f| {
                    write!(f, [when_clauses.format()])?;
                    if let Some(else_clause) = &else_clause {
                        write!(f, [soft_line_break_or_space(), else_clause.format()])?;
                    }
                    Ok(())
                })),
                end_token.format()
            ])]
        )
    }
}
