use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use sql_syntax::PsqlTriggerUpdateOfClause;
use sql_syntax::PsqlTriggerUpdateOfClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerUpdateOfClause;
impl FormatNodeRule<PsqlTriggerUpdateOfClause> for FormatPsqlTriggerUpdateOfClause {
    fn fmt_fields(
        &self,
        node: &PsqlTriggerUpdateOfClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlTriggerUpdateOfClauseFields { of_token, columns } = node.as_fields();

        write_wrapping_fill_clause(of_token.format(), &columns, |_| false, f)
    }
}
