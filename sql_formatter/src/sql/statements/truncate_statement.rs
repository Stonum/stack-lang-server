use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
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

        let keyword = format_with(move |f| {
            write!(f, [truncate_token.format()])?;
            if let Some(table_token) = &table_token {
                write!(f, [space(), table_token.format()])?;
            }
            Ok(())
        });

        write_wrapping_fill_clause(keyword, &tables, |_| false, f)?;

        if let Some(identity_clause) = identity_clause {
            write!(f, [space(), identity_clause.format()])?;
        }
        if let Some(drop_behavior) = drop_behavior {
            write!(f, [space(), drop_behavior.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
