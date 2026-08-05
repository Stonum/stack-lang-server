use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTriggerReferencingItem;
use sql_syntax::SqlTriggerReferencingItemFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTriggerReferencingItem;
impl FormatNodeRule<SqlTriggerReferencingItem> for FormatSqlTriggerReferencingItem {
    fn fmt_fields(
        &self,
        node: &SqlTriggerReferencingItem,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlTriggerReferencingItemFields {
            which_token,
            table_token,
            as_token,
            name,
        } = node.as_fields();

        write!(
            f,
            [
                which_token.format(),
                space(),
                table_token.format(),
                space(),
                as_token.format(),
                space(),
                name.format()
            ]
        )
    }
}
