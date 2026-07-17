use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlDeleteStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDeleteStatement;
impl FormatNodeRule<PsqlDeleteStatement> for FormatPsqlDeleteStatement {
    fn fmt_fields(&self, node: &PsqlDeleteStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
