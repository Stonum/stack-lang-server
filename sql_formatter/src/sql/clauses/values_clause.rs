use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use biome_formatter::write;
use sql_syntax::SqlValuesClause;
use sql_syntax::SqlValuesClauseFields;
use sql_syntax::SqlValuesRow;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlValuesClause;
impl FormatNodeRule<SqlValuesClause> for FormatSqlValuesClause {
    fn fmt_fields(&self, node: &SqlValuesClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlValuesClauseFields {
            with_clause,
            values_token,
            rows,
            semicolon_token,
        } = node.as_fields();

        if let Some(with_clause) = with_clause {
            write!(f, [with_clause.format(), hard_line_break()])?;
        }

        write_wrapping_fill_clause(
            values_token.format(),
            &rows,
            // A row forces its own line if any of its own values is
            // complex -- otherwise short rows of scalars (the common case)
            // pack several per line, same as any other simple list.
            |row: &SqlValuesRow| {
                row.items()
                    .iter()
                    .any(|item| item.is_ok_and(|expr| !is_simple_expression(&expr, 0)))
            },
            f,
        )?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
