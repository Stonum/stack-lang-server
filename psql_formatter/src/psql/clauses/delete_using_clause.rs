use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use psql_syntax::PsqlDeleteUsingClause;
use psql_syntax::PsqlDeleteUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDeleteUsingClause;
impl FormatNodeRule<PsqlDeleteUsingClause> for FormatPsqlDeleteUsingClause {
    fn fmt_fields(&self, node: &PsqlDeleteUsingClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlDeleteUsingClauseFields { using_token, items } = node.as_fields();

        write_wrapping_clause(using_token, &items, f)
    }
}
