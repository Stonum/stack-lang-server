use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::SqlAliasColumnDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAliasColumnDefinitionList;
impl FormatRule<SqlAliasColumnDefinitionList> for FormatSqlAliasColumnDefinitionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlAliasColumnDefinitionList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
