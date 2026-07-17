use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlJoinClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlJoinClause;
impl FormatNodeRule<PsqlJoinClause> for FormatPsqlJoinClause {
    fn fmt_fields(&self, node: &PsqlJoinClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
