use crate::prelude::*;
use psql_syntax::PsqlSelectItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectItemList;
impl FormatRule<PsqlSelectItemList> for FormatPsqlSelectItemList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlSelectItemList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
