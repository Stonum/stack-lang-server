use crate::prelude::*;
use biome_rowan::AstNode;
use psql_syntax::PsqlCaseElseClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCaseElseClause;
impl FormatNodeRule<PsqlCaseElseClause> for FormatPsqlCaseElseClause {
    fn fmt_fields(&self, node: &PsqlCaseElseClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
