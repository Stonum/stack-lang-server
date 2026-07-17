use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlWindowSpecification;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWindowSpecification;
impl FormatNodeRule<PsqlWindowSpecification> for FormatPsqlWindowSpecification {
    fn fmt_fields(
        &self,
        node: &PsqlWindowSpecification,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
