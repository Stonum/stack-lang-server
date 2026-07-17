use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlDeleteUsingClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDeleteUsingClause;
impl FormatNodeRule<PsqlDeleteUsingClause> for FormatPsqlDeleteUsingClause {
    fn fmt_fields(&self, node: &PsqlDeleteUsingClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
