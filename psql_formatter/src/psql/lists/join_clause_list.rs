use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlJoinClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlJoinClauseList;
impl FormatRule<PsqlJoinClauseList> for FormatPsqlJoinClauseList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlJoinClauseList, f: &mut PsqlFormatter) -> FormatResult<()> {
        for join in node.iter() {
            write!(f, [hard_line_break(), join.format()])?;
        }
        Ok(())
    }
}
