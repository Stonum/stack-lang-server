use crate::prelude::*;
use sql_syntax::PsqlTriggerEventList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerEventList;
impl FormatRule<PsqlTriggerEventList> for FormatPsqlTriggerEventList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlTriggerEventList, f: &mut SqlFormatter) -> FormatResult<()> {
        // Always flat (`insert or update or delete`), never one-per-line --
        // real scripts never break this short, fixed-vocabulary list onto
        // multiple lines even when the rest of the trigger header wraps.
        // A `soft_line_break_or_space` here would also be actively wrong:
        // its own fits-check would look past the end of this short list at
        // whatever comes next (a possibly-long `execute function(...)`
        // call), and could decide to wrap the *event list* just because
        // unrelated trailing content doesn't fit on the line.
        f.join_with(space())
            .entries(node.iter().formatted())
            .finish()
    }
}
