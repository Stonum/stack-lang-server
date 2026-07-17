use crate::prelude::*;
use psql_syntax::PsqlTypeArgumentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTypeArgumentList;
impl FormatRule<PsqlTypeArgumentList> for FormatPsqlTypeArgumentList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlTypeArgumentList, f: &mut PsqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
