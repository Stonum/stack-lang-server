use crate::prelude::*;
use sql_syntax::SqlColumnConstraintList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlColumnConstraintList;
impl FormatRule<SqlColumnConstraintList> for FormatSqlColumnConstraintList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlColumnConstraintList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(space())
            .entries(node.iter().formatted())
            .finish()
    }
}
