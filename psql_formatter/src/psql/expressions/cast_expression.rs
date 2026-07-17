use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlCastExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCastExpression;
impl FormatNodeRule<PsqlCastExpression> for FormatPsqlCastExpression {
    fn fmt_fields(&self, node: &PsqlCastExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
