use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlOnConflictClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOnConflictClause;
impl FormatNodeRule<PsqlOnConflictClause> for FormatPsqlOnConflictClause {
    fn fmt_fields(&self, node: &PsqlOnConflictClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
