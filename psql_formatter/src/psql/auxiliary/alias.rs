use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlAlias;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlAlias;
impl FormatNodeRule<PsqlAlias> for FormatPsqlAlias {
    fn fmt_fields(&self, node: &PsqlAlias, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
