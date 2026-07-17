use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlTableName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableName;
impl FormatNodeRule<PsqlTableName> for FormatPsqlTableName {
    fn fmt_fields(&self, node: &PsqlTableName, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
