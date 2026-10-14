use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlInsertStatement;
use sql_syntax::SqlInsertStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlInsertStatement;
impl FormatNodeRule<SqlInsertStatement> for FormatSqlInsertStatement {
    fn fmt_fields(&self, node: &SqlInsertStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlInsertStatementFields {
            with_clause,
            insert_token,
            into_token,
            table,
            columns,
            overriding,
            source,
            on_conflict_clause,
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
                        insert_token.format(),
                        space(),
                        into_token.format(),
                        space(),
                        table.format()
                    ]
                )?;
                if let Some(columns) = &columns {
                    write!(f, [space(), columns.format()])?;
                }
                if let Some(overriding) = overriding {
                    write!(f, [soft_line_break_or_space(), overriding.format()])?;
                }
                write!(f, [soft_line_break_or_space(), source.format()])?;
                if let Some(on_conflict_clause) = on_conflict_clause {
                    write!(f, [soft_line_break_or_space(), on_conflict_clause.format()])?;
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
