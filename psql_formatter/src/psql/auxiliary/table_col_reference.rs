use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlTableColReference;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableColReference;
impl FormatNodeRule<PsqlTableColReference> for FormatPsqlTableColReference {
    fn fmt_fields(&self, node: &PsqlTableColReference, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
