use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlTableBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableBinding;
impl FormatNodeRule<PsqlTableBinding> for FormatPsqlTableBinding {
    fn fmt_fields(&self, node: &PsqlTableBinding, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
