use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlUpdateStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlUpdateStatement;
impl FormatNodeRule<PsqlUpdateStatement> for FormatPsqlUpdateStatement {
    fn fmt_fields(&self, node: &PsqlUpdateStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
