use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlSetItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSetItemList;
impl FormatRule<PsqlSetItemList> for FormatPsqlSetItemList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlSetItemList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
