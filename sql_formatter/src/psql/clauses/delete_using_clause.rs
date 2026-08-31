use crate::prelude::*;
use crate::utils::{is_complex_from_item, write_wrapping_fill_clause};
use sql_syntax::PsqlDeleteUsingClause;
use sql_syntax::PsqlDeleteUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDeleteUsingClause;
impl FormatNodeRule<PsqlDeleteUsingClause> for FormatPsqlDeleteUsingClause {
    fn fmt_fields(&self, node: &PsqlDeleteUsingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlDeleteUsingClauseFields { using_token, items } = node.as_fields();

        write_wrapping_fill_clause(using_token.format(), &items, is_complex_from_item, f)
    }
}
