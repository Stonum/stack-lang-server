use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlWindowFunctionExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWindowFunctionExpression;
impl FormatNodeRule<PsqlWindowFunctionExpression> for FormatPsqlWindowFunctionExpression {
    fn fmt_fields(
        &self,
        node: &PsqlWindowFunctionExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
