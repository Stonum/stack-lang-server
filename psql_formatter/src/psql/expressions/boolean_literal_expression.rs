use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlBooleanLiteralExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBooleanLiteralExpression;
impl FormatNodeRule<PsqlBooleanLiteralExpression> for FormatPsqlBooleanLiteralExpression {
    fn fmt_fields(
        &self,
        node: &PsqlBooleanLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
