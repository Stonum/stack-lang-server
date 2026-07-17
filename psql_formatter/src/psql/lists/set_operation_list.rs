use crate::prelude::*;
use psql_syntax::PsqlSetOperationList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSetOperationList;
impl FormatRule<PsqlSetOperationList> for FormatPsqlSetOperationList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlSetOperationList, f: &mut PsqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
