use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlTypeName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTypeName;
impl FormatNodeRule<PsqlTypeName> for FormatPsqlTypeName {
    fn fmt_fields(&self, node: &PsqlTypeName, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
