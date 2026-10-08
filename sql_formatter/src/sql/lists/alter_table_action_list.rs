use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlAlterTableActionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAlterTableActionList;
impl FormatRule<SqlAlterTableActionList> for FormatSqlAlterTableActionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlAlterTableActionList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
