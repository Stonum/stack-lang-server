use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlLimitClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLimitClause;
impl FormatNodeRule<PsqlLimitClause> for FormatPsqlLimitClause {
    fn fmt_fields(&self, node: &PsqlLimitClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
