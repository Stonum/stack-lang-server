use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlDoUpdateClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDoUpdateClause;
impl FormatNodeRule<PsqlDoUpdateClause> for FormatPsqlDoUpdateClause {
    fn fmt_fields(&self, node: &PsqlDoUpdateClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
