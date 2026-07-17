use crate::prelude::*;
use psql_syntax::PsqlCteDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCteDefinitionList;
impl FormatRule<PsqlCteDefinitionList> for FormatPsqlCteDefinitionList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlCteDefinitionList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
