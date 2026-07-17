use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlParenthesizedExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlParenthesizedExpression;
impl FormatNodeRule<PsqlParenthesizedExpression> for FormatPsqlParenthesizedExpression {
    fn fmt_fields(
        &self,
        node: &PsqlParenthesizedExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
