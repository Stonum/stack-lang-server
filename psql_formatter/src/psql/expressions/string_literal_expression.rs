use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlStringLiteralExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlStringLiteralExpression;
impl FormatNodeRule<PsqlStringLiteralExpression> for FormatPsqlStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &PsqlStringLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
