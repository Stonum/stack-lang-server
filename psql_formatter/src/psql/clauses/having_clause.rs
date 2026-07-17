use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlHavingClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlHavingClause;
impl FormatNodeRule<PsqlHavingClause> for FormatPsqlHavingClause {
    fn fmt_fields(&self, node: &PsqlHavingClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
