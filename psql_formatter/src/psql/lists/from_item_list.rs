use crate::prelude::*;
use crate::utils::write_flat_separated_list;
use psql_syntax::PsqlFromItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFromItemList;
impl FormatRule<PsqlFromItemList> for FormatPsqlFromItemList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlFromItemList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_flat_separated_list(node, f)
    }
}
