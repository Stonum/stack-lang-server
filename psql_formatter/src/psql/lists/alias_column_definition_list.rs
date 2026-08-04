use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlAliasColumnDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlAliasColumnDefinitionList;
impl FormatRule<PsqlAliasColumnDefinitionList> for FormatPsqlAliasColumnDefinitionList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlAliasColumnDefinitionList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
