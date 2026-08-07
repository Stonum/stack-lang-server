use crate::prelude::*;
use sql_syntax::PsqlTriggerReferencingItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerReferencingItemList;
impl FormatRule<PsqlTriggerReferencingItemList> for FormatPsqlTriggerReferencingItemList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlTriggerReferencingItemList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
