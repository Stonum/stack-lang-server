use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlColumnDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlColumnDefinitionList;
impl FormatRule<PsqlColumnDefinitionList> for FormatPsqlColumnDefinitionList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlColumnDefinitionList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
