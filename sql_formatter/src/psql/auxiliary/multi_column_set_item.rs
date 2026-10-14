use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlMultiColumnSetItem;
use sql_syntax::PsqlMultiColumnSetItemFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlMultiColumnSetItem;
impl FormatNodeRule<PsqlMultiColumnSetItem> for FormatPsqlMultiColumnSetItem {
    fn fmt_fields(&self, node: &PsqlMultiColumnSetItem, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlMultiColumnSetItemFields {
            columns,
            eq_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [
                columns.format(),
                space(),
                eq_token.format(),
                space(),
                value.format()
            ]
        )
    }
}
