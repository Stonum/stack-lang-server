use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlUpdateStatement;
use sql_syntax::SqlUpdateStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlUpdateStatement;
impl FormatNodeRule<SqlUpdateStatement> for FormatSqlUpdateStatement {
    fn fmt_fields(&self, node: &SqlUpdateStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlUpdateStatementFields {
            with_clause,
            update_token,
            table,
            set_clause,
            from_clause,
            where_clause,
            returning_clause,
            semicolon_token,
        } = node.as_fields();

        if let Some(with_clause) = with_clause {
            write!(f, [with_clause.format(), hard_line_break()])?;
        }

        write!(f, [update_token.format(), space(), table.format()])?;
        write!(f, [hard_line_break(), set_clause.format()])?;

        if let Some(from_clause) = from_clause {
            write!(f, [hard_line_break(), from_clause.format()])?;
        }
        if let Some(where_clause) = where_clause {
            write!(f, [hard_line_break(), where_clause.format()])?;
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
