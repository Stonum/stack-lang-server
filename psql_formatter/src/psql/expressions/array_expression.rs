use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlArrayExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlArrayExpression;
impl FormatNodeRule<PsqlArrayExpression> for FormatPsqlArrayExpression {
    fn fmt_fields(&self, node: &PsqlArrayExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
