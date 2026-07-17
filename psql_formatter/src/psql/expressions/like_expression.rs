use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlLikeExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLikeExpression;
impl FormatNodeRule<PsqlLikeExpression> for FormatPsqlLikeExpression {
    fn fmt_fields(&self, node: &PsqlLikeExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
