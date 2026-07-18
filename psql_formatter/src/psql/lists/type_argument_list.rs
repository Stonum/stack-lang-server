use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlTypeArgumentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTypeArgumentList;
impl FormatRule<PsqlTypeArgumentList> for FormatPsqlTypeArgumentList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlTypeArgumentList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
