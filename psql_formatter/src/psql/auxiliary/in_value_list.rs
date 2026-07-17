use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlInValueList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlInValueList;
impl FormatNodeRule<PsqlInValueList> for FormatPsqlInValueList {
    fn fmt_fields(&self, node: &PsqlInValueList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
