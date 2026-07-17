use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlSetClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSetClause;
impl FormatNodeRule<PsqlSetClause> for FormatPsqlSetClause {
    fn fmt_fields(&self, node: &PsqlSetClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
