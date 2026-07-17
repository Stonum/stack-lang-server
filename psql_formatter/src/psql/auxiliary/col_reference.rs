use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlColReference;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlColReference;
impl FormatNodeRule<PsqlColReference> for FormatPsqlColReference {
    fn fmt_fields(&self, node: &PsqlColReference, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
