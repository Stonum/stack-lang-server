use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use biome_formatter::write;
use sql_syntax::AnySqlExpression;
use sql_syntax::PsqlPlRaiseStatement;
use sql_syntax::PsqlPlRaiseStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlRaiseStatement;
impl FormatNodeRule<PsqlPlRaiseStatement> for FormatPsqlPlRaiseStatement {
    fn fmt_fields(&self, node: &PsqlPlRaiseStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlRaiseStatementFields {
            raise_token,
            level,
            sqlstate_token,
            arguments,
            using_clause,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(move |f| {
            write!(f, [raise_token.format()])?;
            if let Some(level) = &level {
                write!(f, [space(), level.format()])?;
            }
            if let Some(sqlstate_token) = &sqlstate_token {
                write!(f, [space(), sqlstate_token.format()])?;
            }
            Ok(())
        });

        write!(
            f,
            [group(&format_once(|f| {
                if arguments.is_empty() {
                    write!(f, [keyword])?;
                } else {
                    write_wrapping_fill_clause(
                        keyword,
                        &arguments,
                        |argument: &AnySqlExpression| !is_simple_expression(argument, 0),
                        f,
                    )?;
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
