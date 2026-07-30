use crate::prelude::*;
use psql_syntax::PsqlTriggerEventList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerEventList;
impl FormatRule<PsqlTriggerEventList> for FormatPsqlTriggerEventList {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &PsqlTriggerEventList, f: &mut PsqlFormatter) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
