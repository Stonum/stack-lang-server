use crate::prelude::*;
use crate::utils::write_flat_separated_list;
use psql_syntax::PsqlExpressionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlExpressionList;
impl FormatRule<PsqlExpressionList> for FormatPsqlExpressionList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlExpressionList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_flat_separated_list(node, f)
    }
}
