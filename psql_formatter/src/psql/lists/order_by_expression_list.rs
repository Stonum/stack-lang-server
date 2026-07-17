use crate::prelude::*;
use psql_syntax::PsqlOrderByExpressionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOrderByExpressionList;
impl FormatRule<PsqlOrderByExpressionList> for FormatPsqlOrderByExpressionList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlOrderByExpressionList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
