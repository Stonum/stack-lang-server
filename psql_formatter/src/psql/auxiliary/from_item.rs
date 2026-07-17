use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlFromItem;
use psql_syntax::PsqlFromItemFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFromItem;
impl FormatNodeRule<PsqlFromItem> for FormatPsqlFromItem {
    fn fmt_fields(&self, node: &PsqlFromItem, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlFromItemFields { source, joins } = node.as_fields();

        write!(f, [source.format(), joins.format()])
    }
}
