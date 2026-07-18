use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlInsertStatement;
use psql_syntax::PsqlInsertStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlInsertStatement;
impl FormatNodeRule<PsqlInsertStatement> for FormatPsqlInsertStatement {
    fn fmt_fields(&self, node: &PsqlInsertStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlInsertStatementFields {
            with_clause,
            insert_token,
            into_token,
            table,
            columns,
            source,
            on_conflict_clause,
            returning_clause,
            semicolon_token,
        } = node.as_fields();

        if let Some(with_clause) = with_clause {
            write!(f, [with_clause.format(), hard_line_break()])?;
        }

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
        if let Some(columns) = columns {
            write!(f, [space(), columns.format()])?;
        }

        write!(f, [hard_line_break(), source.format()])?;

        if let Some(on_conflict_clause) = on_conflict_clause {
            write!(f, [hard_line_break(), on_conflict_clause.format()])?;
        }
        if let Some(returning_clause) = returning_clause {
            write!(f, [hard_line_break(), returning_clause.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
