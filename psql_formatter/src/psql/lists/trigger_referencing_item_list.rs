use crate::prelude::*;
use psql_syntax::PsqlTriggerReferencingItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerReferencingItemList;
impl FormatRule<PsqlTriggerReferencingItemList> for FormatPsqlTriggerReferencingItemList {
    type Context = PsqlFormatContext;
    fn fmt(
        &self,
        node: &PsqlTriggerReferencingItemList,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
