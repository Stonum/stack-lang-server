use crate::prelude::*;
use biome_formatter::{format_args, write};
use psql_syntax::PsqlDeleteUsingClause;
use psql_syntax::PsqlDeleteUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDeleteUsingClause;
impl FormatNodeRule<PsqlDeleteUsingClause> for FormatPsqlDeleteUsingClause {
    fn fmt_fields(&self, node: &PsqlDeleteUsingClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlDeleteUsingClauseFields { using_token, items } = node.as_fields();

        // Same cascading-group hazard as `PsqlFromClause` (see Point 4):
        // a single item's own JOINs always hard-break, which would force
        // the wrap-or-not group to always expand.
        if items.len() <= 1 {
            write!(f, [using_token.format(), space(), items.format()])
        } else {
            write!(
                f,
                [group(&format_args![
                    using_token.format(),
                    soft_line_indent_or_space(&items.format())
                ])]
            )
        }
    }
}
