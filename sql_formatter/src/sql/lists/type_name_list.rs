use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlTypeNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTypeNameList;
impl FormatRule<SqlTypeNameList> for FormatSqlTypeNameList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlTypeNameList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
