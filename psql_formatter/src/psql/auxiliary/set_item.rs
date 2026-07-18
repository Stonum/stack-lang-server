use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlSetItem;
use psql_syntax::PsqlSetItemFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSetItem;
impl FormatNodeRule<PsqlSetItem> for FormatPsqlSetItem {
    fn fmt_fields(&self, node: &PsqlSetItem, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSetItemFields {
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
