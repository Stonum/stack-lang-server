use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlSelectExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectExpression;
impl FormatNodeRule<PsqlSelectExpression> for FormatPsqlSelectExpression {
    fn fmt_fields(&self, node: &PsqlSelectExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
