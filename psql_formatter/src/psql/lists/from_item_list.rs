use crate::prelude::*;
use psql_syntax::PsqlFromItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFromItemList;
impl FormatRule<PsqlFromItemList> for FormatPsqlFromItemList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlFromItemList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
