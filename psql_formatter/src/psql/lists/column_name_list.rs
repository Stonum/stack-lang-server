use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlColumnNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlColumnNameList;
impl FormatRule<PsqlColumnNameList> for FormatPsqlColumnNameList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlColumnNameList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
