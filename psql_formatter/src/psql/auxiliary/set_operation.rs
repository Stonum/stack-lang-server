use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlSetOperation;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSetOperation;
impl FormatNodeRule<PsqlSetOperation> for FormatPsqlSetOperation {
    fn fmt_fields(&self, node: &PsqlSetOperation, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
