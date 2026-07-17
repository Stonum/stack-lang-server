use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlGroupByItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlGroupByItemList;
impl FormatRule<PsqlGroupByItemList> for FormatPsqlGroupByItemList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlGroupByItemList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
