use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlTableNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableNameList;
impl FormatRule<PsqlTableNameList> for FormatPsqlTableNameList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlTableNameList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
