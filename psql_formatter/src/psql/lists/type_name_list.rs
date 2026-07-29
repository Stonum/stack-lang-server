use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlTypeNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTypeNameList;
impl FormatRule<PsqlTypeNameList> for FormatPsqlTypeNameList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlTypeNameList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
