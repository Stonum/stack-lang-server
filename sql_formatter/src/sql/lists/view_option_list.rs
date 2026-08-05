use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlViewOptionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlViewOptionList;
impl FormatRule<SqlViewOptionList> for FormatSqlViewOptionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlViewOptionList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
