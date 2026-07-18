use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use psql_syntax::PsqlGroupByClause;
use psql_syntax::PsqlGroupByClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlGroupByClause;
impl FormatNodeRule<PsqlGroupByClause> for FormatPsqlGroupByClause {
    fn fmt_fields(&self, node: &PsqlGroupByClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlGroupByClauseFields {
            group_by_token,
            items,
        } = node.as_fields();

        write_wrapping_clause(group_by_token, &items, f)
    }
}
