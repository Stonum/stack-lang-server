use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlTypeArgumentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTypeArgumentList;
impl FormatRule<SqlTypeArgumentList> for FormatSqlTypeArgumentList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlTypeArgumentList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
