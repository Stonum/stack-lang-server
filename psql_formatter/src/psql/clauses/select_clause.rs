use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use psql_syntax::PsqlSelectClause;
use psql_syntax::PsqlSelectClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectClause;
impl FormatNodeRule<PsqlSelectClause> for FormatPsqlSelectClause {
    fn fmt_fields(&self, node: &PsqlSelectClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSelectClauseFields { select_token, list } = node.as_fields();

        write_wrapping_clause(select_token, &list, f)
    }
}
