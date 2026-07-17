use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlColumnList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlColumnList;
impl FormatNodeRule<PsqlColumnList> for FormatPsqlColumnList {
    fn fmt_fields(&self, node: &PsqlColumnList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
