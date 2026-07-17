use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlOrderByExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOrderByExpression;
impl FormatNodeRule<PsqlOrderByExpression> for FormatPsqlOrderByExpression {
    fn fmt_fields(&self, node: &PsqlOrderByExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
