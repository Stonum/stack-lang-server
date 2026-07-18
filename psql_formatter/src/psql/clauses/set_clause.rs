use crate::prelude::*;
use crate::utils::write_wrapping_clause;
use psql_syntax::PsqlSetClause;
use psql_syntax::PsqlSetClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSetClause;
impl FormatNodeRule<PsqlSetClause> for FormatPsqlSetClause {
    fn fmt_fields(&self, node: &PsqlSetClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSetClauseFields { set_token, items } = node.as_fields();

        write_wrapping_clause(set_token, &items, f)
    }
}
