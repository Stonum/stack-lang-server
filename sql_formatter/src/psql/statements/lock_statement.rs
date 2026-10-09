use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::PsqlLockStatement;
use sql_syntax::PsqlLockStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLockStatement;
impl FormatNodeRule<PsqlLockStatement> for FormatPsqlLockStatement {
    fn fmt_fields(&self, node: &PsqlLockStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlLockStatementFields {
            lock_token,
            table_token,
            only_token,
            tables,
            mode,
            nowait_token,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(|f| {
            write!(f, [lock_token.format()])?;
            for token in [&table_token, &only_token].into_iter().flatten() {
                write!(f, [space(), token.format()])?;
            }
            Ok(())
        });

        // Once the table list wraps, the lock mode and `NOWAIT` move to their own lines.
        write!(
            f,
            [group(&format_with(|f: &mut SqlFormatter| {
                write_wrapping_fill_clause(&keyword, &tables, |_| false, f)?;
                if let Some(mode) = &mode {
                    write!(f, [soft_line_break_or_space(), mode.format()])?;
                }
                if let Some(nowait_token) = &nowait_token {
                    write!(f, [soft_line_break_or_space(), nowait_token.format()])?;
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
