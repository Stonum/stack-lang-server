use crate::prelude::*;
use psql_syntax::PsqlGroupByItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlGroupByItemList;
impl FormatRule<PsqlGroupByItemList> for FormatPsqlGroupByItemList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlGroupByItemList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
