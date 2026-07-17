use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlUnaryExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlUnaryExpression;
impl FormatNodeRule<PsqlUnaryExpression> for FormatPsqlUnaryExpression {
    fn fmt_fields(&self, node: &PsqlUnaryExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
