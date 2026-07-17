use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlFunctionBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFunctionBinding;
impl FormatNodeRule<PsqlFunctionBinding> for FormatPsqlFunctionBinding {
    fn fmt_fields(&self, node: &PsqlFunctionBinding, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
