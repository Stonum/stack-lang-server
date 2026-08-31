use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use biome_formatter::write;
use sql_syntax::SqlCreateTableStatement;
use sql_syntax::SqlCreateTableStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCreateTableStatement;
impl FormatNodeRule<SqlCreateTableStatement> for FormatSqlCreateTableStatement {
    fn fmt_fields(&self, node: &SqlCreateTableStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCreateTableStatementFields {
            create_token,
            table_token,
            if_token,
            not_token,
            exists_token,
            name,
            l_paren_token,
            columns,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [create_token.format(), space(), table_token.format()])?;

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
        write_bracketed_fill_list(l_paren_token, &columns, r_paren_token, |_| false, f)?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
