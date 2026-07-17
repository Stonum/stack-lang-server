use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlInsertValues;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlInsertValues;
impl FormatNodeRule<PsqlInsertValues> for FormatPsqlInsertValues {
    fn fmt_fields(&self, node: &PsqlInsertValues, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
