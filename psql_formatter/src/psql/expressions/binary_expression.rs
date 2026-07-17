use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlBinaryExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBinaryExpression;
impl FormatNodeRule<PsqlBinaryExpression> for FormatPsqlBinaryExpression {
    fn fmt_fields(&self, node: &PsqlBinaryExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
