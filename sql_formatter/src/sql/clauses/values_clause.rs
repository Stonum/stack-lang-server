use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use biome_formatter::write;
use sql_syntax::SqlValuesClause;
use sql_syntax::SqlValuesClauseFields;
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

        write_wrapping_clause(values_token, &rows, f)?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
