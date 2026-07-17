use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlRoot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlRoot;
impl FormatNodeRule<PsqlRoot> for FormatPsqlRoot {
    fn fmt_fields(&self, node: &PsqlRoot, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
