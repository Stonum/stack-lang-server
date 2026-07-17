use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlNullLiteralExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlNullLiteralExpression;
impl FormatNodeRule<PsqlNullLiteralExpression> for FormatPsqlNullLiteralExpression {
    fn fmt_fields(
        &self,
        node: &PsqlNullLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
