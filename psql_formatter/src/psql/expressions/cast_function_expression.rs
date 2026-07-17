use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlCastFunctionExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCastFunctionExpression;
impl FormatNodeRule<PsqlCastFunctionExpression> for FormatPsqlCastFunctionExpression {
    fn fmt_fields(
        &self,
        node: &PsqlCastFunctionExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
