use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlLogicalExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLogicalExpression;
impl FormatNodeRule<PsqlLogicalExpression> for FormatPsqlLogicalExpression {
    fn fmt_fields(&self, node: &PsqlLogicalExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
