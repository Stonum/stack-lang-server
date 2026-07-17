use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlWhereClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWhereClause;
impl FormatNodeRule<PsqlWhereClause> for FormatPsqlWhereClause {
    fn fmt_fields(&self, node: &PsqlWhereClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
