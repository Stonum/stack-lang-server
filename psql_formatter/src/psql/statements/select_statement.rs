use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlSelectStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectStatement;
impl FormatNodeRule<PsqlSelectStatement> for FormatPsqlSelectStatement {
    fn fmt_fields(&self, node: &PsqlSelectStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
