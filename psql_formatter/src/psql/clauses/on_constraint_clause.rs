use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlOnConstraintClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOnConstraintClause;
impl FormatNodeRule<PsqlOnConstraintClause> for FormatPsqlOnConstraintClause {
    fn fmt_fields(&self, node: &PsqlOnConstraintClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
