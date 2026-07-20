use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use psql_syntax::AnyPsqlSelectItem;
use psql_syntax::PsqlSelectClause;
use psql_syntax::PsqlSelectClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectClause;
impl FormatNodeRule<PsqlSelectClause> for FormatPsqlSelectClause {
    fn fmt_fields(&self, node: &PsqlSelectClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSelectClauseFields { select_token, list } = node.as_fields();

        write_wrapping_fill_clause(
            select_token,
            &list,
            |item: &AnyPsqlSelectItem| match item {
                AnyPsqlSelectItem::PsqlSelectExpression(item) => item
                    .expr()
                    .is_ok_and(|expr| !is_simple_expression(&expr, 0)),
                AnyPsqlSelectItem::PsqlStar(_) => false,
            },
            f,
        )
    }
}
