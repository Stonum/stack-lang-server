use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlGranteeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlGranteeList;
impl FormatRule<SqlGranteeList> for FormatSqlGranteeList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlGranteeList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
