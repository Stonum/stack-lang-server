use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTriggerReferencingItem;
use psql_syntax::PsqlTriggerReferencingItemFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerReferencingItem;
impl FormatNodeRule<PsqlTriggerReferencingItem> for FormatPsqlTriggerReferencingItem {
    fn fmt_fields(
        &self,
        node: &PsqlTriggerReferencingItem,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlTriggerReferencingItemFields {
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
