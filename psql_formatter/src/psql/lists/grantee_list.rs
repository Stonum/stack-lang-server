use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlGranteeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlGranteeList;
impl FormatRule<PsqlGranteeList> for FormatPsqlGranteeList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlGranteeList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
