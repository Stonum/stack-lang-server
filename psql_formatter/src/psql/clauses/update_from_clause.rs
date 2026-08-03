use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use psql_syntax::PsqlUpdateFromClause;
use psql_syntax::PsqlUpdateFromClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlUpdateFromClause;
impl FormatNodeRule<PsqlUpdateFromClause> for FormatPsqlUpdateFromClause {
    fn fmt_fields(&self, node: &PsqlUpdateFromClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlUpdateFromClauseFields { from_token, items } = node.as_fields();

        write_wrapping_clause(from_token, &items, f)
    }
}
