use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSetItem;
use sql_syntax::SqlSetItemFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSetItem;
impl FormatNodeRule<SqlSetItem> for FormatSqlSetItem {
    fn fmt_fields(&self, node: &SqlSetItem, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSetItemFields {
            column,
            eq_token,
            expr,
        } = node.as_fields();

        write!(
            f,
            [
                column.format(),
                space(),
                eq_token.format(),
                space(),
                expr.format()
            ]
        )
    }
}
