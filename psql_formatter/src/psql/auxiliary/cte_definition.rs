use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlCteDefinition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCteDefinition;
impl FormatNodeRule<PsqlCteDefinition> for FormatPsqlCteDefinition {
    fn fmt_fields(&self, node: &PsqlCteDefinition, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
