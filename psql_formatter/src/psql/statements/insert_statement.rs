use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlInsertStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlInsertStatement;
impl FormatNodeRule<PsqlInsertStatement> for FormatPsqlInsertStatement {
    fn fmt_fields(&self, node: &PsqlInsertStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
