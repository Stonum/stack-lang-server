use crate::prelude::*;
use psql_syntax::PsqlExpressionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlExpressionList;
impl FormatRule<PsqlExpressionList> for FormatPsqlExpressionList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlExpressionList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
