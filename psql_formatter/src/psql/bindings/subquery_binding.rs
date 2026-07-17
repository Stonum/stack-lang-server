use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlSubqueryBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSubqueryBinding;
impl FormatNodeRule<PsqlSubqueryBinding> for FormatPsqlSubqueryBinding {
    fn fmt_fields(&self, node: &PsqlSubqueryBinding, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
