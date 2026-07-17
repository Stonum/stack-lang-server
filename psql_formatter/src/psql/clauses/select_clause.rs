use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlSelectClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectClause;
impl FormatNodeRule<PsqlSelectClause> for FormatPsqlSelectClause {
    fn fmt_fields(&self, node: &PsqlSelectClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
