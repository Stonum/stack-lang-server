use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlCaseExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCaseExpression;
impl FormatNodeRule<PsqlCaseExpression> for FormatPsqlCaseExpression {
    fn fmt_fields(&self, node: &PsqlCaseExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
