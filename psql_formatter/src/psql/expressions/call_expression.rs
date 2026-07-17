use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlCallExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCallExpression;
impl FormatNodeRule<PsqlCallExpression> for FormatPsqlCallExpression {
    fn fmt_fields(&self, node: &PsqlCallExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
