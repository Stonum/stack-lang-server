use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlCteDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCteDefinitionList;
impl FormatRule<SqlCteDefinitionList> for FormatSqlCteDefinitionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlCteDefinitionList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
