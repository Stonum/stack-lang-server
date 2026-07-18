use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use psql_syntax::PsqlReturningClause;
use psql_syntax::PsqlReturningClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturningClause;
impl FormatNodeRule<PsqlReturningClause> for FormatPsqlReturningClause {
    fn fmt_fields(&self, node: &PsqlReturningClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlReturningClauseFields {
            returning_token,
            items,
        } = node.as_fields();

        write_wrapping_clause(returning_token, &items, f)
    }
}
