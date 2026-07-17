use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlStar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlStar;
impl FormatNodeRule<PsqlStar> for FormatPsqlStar {
    fn fmt_fields(&self, node: &PsqlStar, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
