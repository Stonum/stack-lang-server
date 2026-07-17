use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlFromItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFromItem;
impl FormatNodeRule<PsqlFromItem> for FormatPsqlFromItem {
    fn fmt_fields(&self, node: &PsqlFromItem, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
