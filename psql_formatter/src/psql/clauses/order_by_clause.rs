use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlOrderByClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOrderByClause;
impl FormatNodeRule<PsqlOrderByClause> for FormatPsqlOrderByClause {
    fn fmt_fields(&self, node: &PsqlOrderByClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
