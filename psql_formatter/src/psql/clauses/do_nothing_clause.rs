use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlDoNothingClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDoNothingClause;
impl FormatNodeRule<PsqlDoNothingClause> for FormatPsqlDoNothingClause {
    fn fmt_fields(&self, node: &PsqlDoNothingClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
