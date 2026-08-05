use crate::prelude::*;
use biome_rowan::AstNode;
use sql_syntax::SqlEmptyStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlEmptyStatement;
impl FormatNodeRule<SqlEmptyStatement> for FormatSqlEmptyStatement {
    fn fmt_fields(&self, node: &SqlEmptyStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
