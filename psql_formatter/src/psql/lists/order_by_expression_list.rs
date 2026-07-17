use crate::prelude::*;
use crate::utils::write_flat_separated_list;
use psql_syntax::PsqlOrderByExpressionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOrderByExpressionList;
impl FormatRule<PsqlOrderByExpressionList> for FormatPsqlOrderByExpressionList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlOrderByExpressionList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_flat_separated_list(node, f)
    }
}
