use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlCaseWhenClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCaseWhenClause;
impl FormatNodeRule<PsqlCaseWhenClause> for FormatPsqlCaseWhenClause {
    fn fmt_fields(&self, node: &PsqlCaseWhenClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
