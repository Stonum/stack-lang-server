use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlFromItem;
use sql_syntax::SqlFromItemFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFromItem;
impl FormatNodeRule<SqlFromItem> for FormatSqlFromItem {
    fn fmt_fields(&self, node: &SqlFromItem, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlFromItemFields { source, joins } = node.as_fields();

        write!(f, [source.format(), joins.format()])
    }
}
