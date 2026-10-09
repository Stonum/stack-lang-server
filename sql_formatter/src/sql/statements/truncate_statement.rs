use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::{format_args, write};
use sql_syntax::SqlTruncateStatement;
use sql_syntax::SqlTruncateStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTruncateStatement;
impl FormatNodeRule<SqlTruncateStatement> for FormatSqlTruncateStatement {
    fn fmt_fields(&self, node: &SqlTruncateStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTruncateStatementFields {
            truncate_token,
            table_token,
            tables,
            identity_clause,
            drop_behavior,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(|f| {
            write!(f, [truncate_token.format()])?;
            if let Some(table_token) = &table_token {
                write!(f, [space(), table_token.format()])?;
            }
            Ok(())
        });
        let list = format_with(|f| write_wrapping_fill_clause(&keyword, &tables, |_| false, f));

        if identity_clause.is_none() && drop_behavior.is_none() {
            write!(f, [list])?;
        } else {
            // Once the table list wraps, the trailing options move to their
            // own line instead of hanging off its last row.
            let tail = format_with(|f| {
                let mut separator = false;
                if let Some(identity_clause) = &identity_clause {
                    write!(f, [identity_clause.format()])?;
                    separator = true;
                }
                if let Some(drop_behavior) = &drop_behavior {
                    if separator {
                        write!(f, [space()])?;
                    }
                    write!(f, [drop_behavior.format()])?;
                }
                Ok(())
            });
            write!(
                f,
                [group(&format_args![list, soft_line_break_or_space(), tail])]
            )?;
        }

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
