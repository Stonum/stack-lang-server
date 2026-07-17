use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlSetItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSetItem;
impl FormatNodeRule<PsqlSetItem> for FormatPsqlSetItem {
    fn fmt_fields(&self, node: &PsqlSetItem, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
