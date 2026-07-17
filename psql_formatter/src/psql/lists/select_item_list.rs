use crate::prelude::*;
use crate::utils::write_flat_separated_list;
use psql_syntax::PsqlSelectItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectItemList;
impl FormatRule<PsqlSelectItemList> for FormatPsqlSelectItemList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlSelectItemList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_flat_separated_list(node, f)
    }
}
