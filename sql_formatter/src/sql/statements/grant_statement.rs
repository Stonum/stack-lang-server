use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::SqlGrantStatement;
use sql_syntax::SqlGrantStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlGrantStatement;
impl FormatNodeRule<SqlGrantStatement> for FormatSqlGrantStatement {
    fn fmt_fields(&self, node: &SqlGrantStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlGrantStatementFields {
            grant_token,
            privileges,
            target,
            to_token,
            grantees,
            grant_option,
            semicolon_token,
        } = node.as_fields();

        write!(f, [grant_token.format(), space(), privileges.format()])?;

        // Once the object list wraps, `TO ...` moves to its own line.
        write!(
            f,
            [group(&format_with(|f: &mut SqlFormatter| {
                if let Some(target) = &target {
                    write!(f, [space(), target.format(), soft_line_break_or_space()])?;
                } else {
                    write!(f, [space()])?;
                }
                write_wrapping_fill_clause(to_token.format(), &grantees, |_| false, f)?;
                if let Some(grant_option) = &grant_option {
                    write!(f, [space(), grant_option.format()])?;
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
