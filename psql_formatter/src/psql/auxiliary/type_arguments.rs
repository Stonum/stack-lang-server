use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlTypeArguments;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTypeArguments;
impl FormatNodeRule<PsqlTypeArguments> for FormatPsqlTypeArguments {
    fn fmt_fields(&self, node: &PsqlTypeArguments, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
