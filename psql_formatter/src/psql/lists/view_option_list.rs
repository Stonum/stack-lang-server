use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlViewOptionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlViewOptionList;
impl FormatRule<PsqlViewOptionList> for FormatPsqlViewOptionList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlViewOptionList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
