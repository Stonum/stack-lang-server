use crate::prelude::*;
use psql_syntax::PsqlJoinClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlJoinClauseList;
impl FormatRule<PsqlJoinClauseList> for FormatPsqlJoinClauseList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlJoinClauseList, f: &mut PsqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
