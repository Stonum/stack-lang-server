use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlIsNullExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlIsNullExpression;
impl FormatNodeRule<PsqlIsNullExpression> for FormatPsqlIsNullExpression {
    fn fmt_fields(&self, node: &PsqlIsNullExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
