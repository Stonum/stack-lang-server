use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlTildeArraySuffix;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTildeArraySuffix;
impl FormatNodeRule<PsqlTildeArraySuffix> for FormatPsqlTildeArraySuffix {
    fn fmt_fields(&self, node: &PsqlTildeArraySuffix, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
