use crate::prelude::*;
use psql_syntax::PsqlStatementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlStatementList;
impl FormatRule<PsqlStatementList> for FormatPsqlStatementList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlStatementList, f: &mut PsqlFormatter) -> FormatResult<()> {
        f.join_with(hard_line_break())
            .entries(node.iter().formatted())
            .finish()
    }
}
