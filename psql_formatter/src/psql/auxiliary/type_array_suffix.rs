use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlTypeArraySuffix;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTypeArraySuffix;
impl FormatNodeRule<PsqlTypeArraySuffix> for FormatPsqlTypeArraySuffix {
    fn fmt_fields(&self, node: &PsqlTypeArraySuffix, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
