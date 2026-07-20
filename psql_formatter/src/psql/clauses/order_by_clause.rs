use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use psql_syntax::PsqlOrderByClause;
use psql_syntax::PsqlOrderByClauseFields;
use psql_syntax::PsqlOrderByExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOrderByClause;
impl FormatNodeRule<PsqlOrderByClause> for FormatPsqlOrderByClause {
    fn fmt_fields(&self, node: &PsqlOrderByClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlOrderByClauseFields {
            order_by_token,
            items,
        } = node.as_fields();

        write_wrapping_fill_clause(
            order_by_token,
            &items,
            |item: &PsqlOrderByExpression| {
                item.item()
                    .is_ok_and(|expr| !is_simple_expression(&expr, 0))
            },
            f,
        )
    }
}
