use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlArraySubscriptExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlArraySubscriptExpression;
impl FormatNodeRule<PsqlArraySubscriptExpression> for FormatPsqlArraySubscriptExpression {
    fn fmt_fields(
        &self,
        node: &PsqlArraySubscriptExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
