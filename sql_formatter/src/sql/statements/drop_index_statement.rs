use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::{format_args, write};
use sql_syntax::SqlDropIndexStatement;
use sql_syntax::SqlDropIndexStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDropIndexStatement;
impl FormatNodeRule<SqlDropIndexStatement> for FormatSqlDropIndexStatement {
    fn fmt_fields(&self, node: &SqlDropIndexStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDropIndexStatementFields {
            drop_token,
            index_token,
            concurrently_token,
            if_token,
            exists_token,
            indexes,
            drop_behavior,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(|f| {
            write!(f, [drop_token.format(), space(), index_token.format()])?;
            for token in [&concurrently_token, &if_token, &exists_token]
                .into_iter()
                .flatten()
            {
                write!(f, [space(), token.format()])?;
            }
            Ok(())
        });
        let list = format_with(|f| write_wrapping_fill_clause(&keyword, &indexes, |_| false, f));

        match drop_behavior {
            // Once the index list wraps, `CASCADE`/`RESTRICT` moves to its own line.
            Some(drop_behavior) => write!(
                f,
                [group(&format_args![
                    list,
                    soft_line_break_or_space(),
                    drop_behavior.format()
                ])]
            )?,
            None => write!(f, [list])?,
        }

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
