use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlOffsetClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOffsetClause;
impl FormatNodeRule<PsqlOffsetClause> for FormatPsqlOffsetClause {
    fn fmt_fields(&self, node: &PsqlOffsetClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
