use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlDataBaseName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDataBaseName;
impl FormatNodeRule<PsqlDataBaseName> for FormatPsqlDataBaseName {
    fn fmt_fields(&self, node: &PsqlDataBaseName, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
