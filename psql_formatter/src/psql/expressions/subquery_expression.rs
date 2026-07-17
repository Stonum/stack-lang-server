use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlSubqueryExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSubqueryExpression;
impl FormatNodeRule<PsqlSubqueryExpression> for FormatPsqlSubqueryExpression {
    fn fmt_fields(&self, node: &PsqlSubqueryExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
