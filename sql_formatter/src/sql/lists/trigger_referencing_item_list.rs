use crate::prelude::*;
use sql_syntax::SqlTriggerReferencingItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTriggerReferencingItemList;
impl FormatRule<SqlTriggerReferencingItemList> for FormatSqlTriggerReferencingItemList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlTriggerReferencingItemList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
