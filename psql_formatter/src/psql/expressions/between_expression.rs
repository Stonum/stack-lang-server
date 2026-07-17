use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlBetweenExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBetweenExpression;
impl FormatNodeRule<PsqlBetweenExpression> for FormatPsqlBetweenExpression {
    fn fmt_fields(&self, node: &PsqlBetweenExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
