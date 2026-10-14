use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use sql_syntax::AnySqlSetItem;
use sql_syntax::SqlSetClause;
use sql_syntax::SqlSetClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSetClause;
impl FormatNodeRule<SqlSetClause> for FormatSqlSetClause {
    fn fmt_fields(&self, node: &SqlSetClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSetClauseFields { set_token, items } = node.as_fields();

        write_wrapping_fill_clause(
            set_token.format(),
            &items,
            |item: &AnySqlSetItem| match item {
                AnySqlSetItem::SqlSetItem(item) => item
                    .expr()
                    .is_ok_and(|expr| !is_simple_expression(&expr, 0)),
                AnySqlSetItem::PsqlMultiColumnSetItem(_) => true,
            },
            f,
        )
    }
}
