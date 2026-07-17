use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlNumberLiteralExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlNumberLiteralExpression;
impl FormatNodeRule<PsqlNumberLiteralExpression> for FormatPsqlNumberLiteralExpression {
    fn fmt_fields(
        &self,
        node: &PsqlNumberLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
