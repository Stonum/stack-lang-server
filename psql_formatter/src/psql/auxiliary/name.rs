use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlName;
impl FormatNodeRule<PsqlName> for FormatPsqlName {
    fn fmt_fields(&self, node: &PsqlName, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
