use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use sql_syntax::PsqlTypeNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTypeNameList;
impl FormatRule<PsqlTypeNameList> for FormatPsqlTypeNameList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlTypeNameList, f: &mut SqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
