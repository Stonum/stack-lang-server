use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::SqlRevokeStatement;
use sql_syntax::SqlRevokeStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlRevokeStatement;
impl FormatNodeRule<SqlRevokeStatement> for FormatSqlRevokeStatement {
    fn fmt_fields(&self, node: &SqlRevokeStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlRevokeStatementFields {
            revoke_token,
            grant_option_for,
            privileges,
            target,
            from_token,
            grantees,
            drop_behavior,
            semicolon_token,
        } = node.as_fields();

        write!(f, [revoke_token.format()])?;
        if let Some(grant_option_for) = grant_option_for {
            write!(f, [space(), grant_option_for.format()])?;
        }
        write!(f, [space(), privileges.format()])?;

        // Once the object list wraps, `FROM ...` moves to its own line, and
        // once anything wraps, so does `CASCADE`/`RESTRICT`.
        write!(
            f,
            [group(&format_with(|f: &mut SqlFormatter| {
                if let Some(target) = &target {
                    write!(f, [space(), target.format(), soft_line_break_or_space()])?;
                } else {
                    write!(f, [space()])?;
                }
                write_wrapping_fill_clause(from_token.format(), &grantees, |_| false, f)?;
                if let Some(drop_behavior) = &drop_behavior {
                    write!(f, [soft_line_break_or_space(), drop_behavior.format()])?;
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
