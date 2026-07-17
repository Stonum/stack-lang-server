use crate::prelude::*;
use psql_syntax::PsqlColumnNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlColumnNameList;
impl FormatRule<PsqlColumnNameList> for FormatPsqlColumnNameList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlColumnNameList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
