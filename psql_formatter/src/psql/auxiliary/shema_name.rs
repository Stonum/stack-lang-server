use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlShemaName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlShemaName;
impl FormatNodeRule<PsqlShemaName> for FormatPsqlShemaName {
    fn fmt_fields(&self, node: &PsqlShemaName, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
