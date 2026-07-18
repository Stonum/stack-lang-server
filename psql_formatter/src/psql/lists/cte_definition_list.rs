use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlCteDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCteDefinitionList;
impl FormatRule<PsqlCteDefinitionList> for FormatPsqlCteDefinitionList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlCteDefinitionList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
