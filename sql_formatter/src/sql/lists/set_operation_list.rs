use crate::prelude::*;
use sql_syntax::SqlSetOperationList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSetOperationList;
impl FormatRule<SqlSetOperationList> for FormatSqlSetOperationList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlSetOperationList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
