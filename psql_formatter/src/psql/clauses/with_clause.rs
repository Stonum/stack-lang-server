use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlWithClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWithClause;
impl FormatNodeRule<PsqlWithClause> for FormatPsqlWithClause {
    fn fmt_fields(&self, node: &PsqlWithClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
