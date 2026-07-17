use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlFromClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFromClause;
impl FormatNodeRule<PsqlFromClause> for FormatPsqlFromClause {
    fn fmt_fields(&self, node: &PsqlFromClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
