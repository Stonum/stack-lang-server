use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlInExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlInExpression;
impl FormatNodeRule<PsqlInExpression> for FormatPsqlInExpression {
    fn fmt_fields(&self, node: &PsqlInExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
