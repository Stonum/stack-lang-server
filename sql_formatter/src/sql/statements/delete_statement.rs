use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDeleteStatement;
use sql_syntax::SqlDeleteStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDeleteStatement;
impl FormatNodeRule<SqlDeleteStatement> for FormatSqlDeleteStatement {
    fn fmt_fields(&self, node: &SqlDeleteStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDeleteStatementFields {
            with_clause,
            delete_token,
            from_token,
            table,
            using,
            where_clause,
            returning_clause,
            semicolon_token,
        } = node.as_fields();

        if let Some(with_clause) = with_clause {
            write!(f, [with_clause.format(), hard_line_break()])?;
        }

        // Same as SELECT: a short statement stays on one line, anything that
        // doesn't fit (or breaks inside) puts each clause on its own line.
        write!(
            f,
            [group(&format_once(|f| {
                write!(
                    f,
                    [
                        delete_token.format(),
                        space(),
                        from_token.format(),
                        space(),
                        table.format()
                    ]
                )?;
                if let Some(using) = using {
                    write!(f, [soft_line_break_or_space(), using.format()])?;
                }
                if let Some(where_clause) = where_clause {
                    write!(f, [soft_line_break_or_space(), where_clause.format()])?;
                }
                if let Some(returning_clause) = returning_clause {
                    write!(f, [soft_line_break_or_space(), returning_clause.format()])?;
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
