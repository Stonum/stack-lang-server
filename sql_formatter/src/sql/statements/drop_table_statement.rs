use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::SqlDropTableStatement;
use sql_syntax::SqlDropTableStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDropTableStatement;
impl FormatNodeRule<SqlDropTableStatement> for FormatSqlDropTableStatement {
    fn fmt_fields(&self, node: &SqlDropTableStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDropTableStatementFields {
            drop_token,
            table_token,
            if_token,
            exists_token,
            tables,
            drop_behavior,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(move |f| {
            write!(f, [drop_token.format(), space(), table_token.format()])?;
            if let Some(if_token) = &if_token {
                write!(f, [space(), if_token.format()])?;
            }
            if let Some(exists_token) = &exists_token {
                write!(f, [space(), exists_token.format()])?;
            }
            Ok(())
        });

        write_wrapping_fill_clause(keyword, &tables, |_| false, f)?;

        if let Some(drop_behavior) = drop_behavior {
            write!(f, [space(), drop_behavior.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
