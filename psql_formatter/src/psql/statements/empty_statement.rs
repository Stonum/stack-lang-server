use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlEmptyStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlEmptyStatement;
impl FormatNodeRule<PsqlEmptyStatement> for FormatPsqlEmptyStatement {
    fn fmt_fields(&self, node: &PsqlEmptyStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
