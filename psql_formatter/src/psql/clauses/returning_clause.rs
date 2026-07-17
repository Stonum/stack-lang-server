use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlReturningClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturningClause;
impl FormatNodeRule<PsqlReturningClause> for FormatPsqlReturningClause {
    fn fmt_fields(&self, node: &PsqlReturningClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
