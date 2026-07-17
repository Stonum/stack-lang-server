use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlTildeName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTildeName;
impl FormatNodeRule<PsqlTildeName> for FormatPsqlTildeName {
    fn fmt_fields(&self, node: &PsqlTildeName, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
