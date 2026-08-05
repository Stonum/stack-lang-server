use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use biome_formatter::write;
use psql_syntax::PsqlValuesClause;
use psql_syntax::PsqlValuesClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlValuesClause;
impl FormatNodeRule<PsqlValuesClause> for FormatPsqlValuesClause {
    fn fmt_fields(&self, node: &PsqlValuesClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlValuesClauseFields {
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
