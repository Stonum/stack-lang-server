use crate::prelude::*;

use biome_formatter::write;
use sql_syntax::SqlCreateTableStatement;
use sql_syntax::SqlCreateTableStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCreateTableStatement;
impl FormatNodeRule<SqlCreateTableStatement> for FormatSqlCreateTableStatement {
    fn fmt_fields(&self, node: &SqlCreateTableStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCreateTableStatementFields {
            create_token,
            persistence,
            table_token,
            if_token,
            not_token,
            exists_token,
            name,
            l_paren_token,
            elements,
            r_paren_token,
            on_commit,
            semicolon_token,
        } = node.as_fields();

        write!(f, [create_token.format()])?;
        if let Some(persistence) = persistence {
            write!(f, [space(), persistence.format()])?;
        }
        write!(f, [space(), table_token.format()])?;

        if let Some(if_token) = if_token {
            write!(f, [space(), if_token.format()])?;
        }
        if let Some(not_token) = not_token {
            write!(f, [space(), not_token.format()])?;
        }
        if let Some(exists_token) = exists_token {
            write!(f, [space(), exists_token.format()])?;
        }

        write!(f, [space(), name.format(), space()])?;
        // One column or constraint per line once the list wraps, even
        // when there is a single element that breaks on its own.
        write!(
            f,
            [
                l_paren_token.format(),
                group(&soft_block_indent(&elements.format())),
                r_paren_token.format()
            ]
        )?;

        if let Some(on_commit) = on_commit {
            write!(f, [space(), on_commit.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
