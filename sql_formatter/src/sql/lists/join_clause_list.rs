use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlJoinClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlJoinClauseList;
impl FormatRule<SqlJoinClauseList> for FormatSqlJoinClauseList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlJoinClauseList, f: &mut SqlFormatter) -> FormatResult<()> {
        for join in node.iter() {
            write!(f, [hard_line_break(), join.format()])?;
        }
        Ok(())
    }
}
