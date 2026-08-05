use crate::prelude::*;
use crate::utils::write_wrapping_separated_list;
use psql_syntax::PsqlValuesRowList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlValuesRowList;
impl FormatRule<PsqlValuesRowList> for FormatPsqlValuesRowList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlValuesRowList, f: &mut PsqlFormatter) -> FormatResult<()> {
        write_wrapping_separated_list(node, f)
    }
}
