use crate::prelude::*;
use biome_formatter::{format_args, write};
use psql_syntax::PsqlFromClause;
use psql_syntax::PsqlFromClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFromClause;
impl FormatNodeRule<PsqlFromClause> for FormatPsqlFromClause {
    fn fmt_fields(&self, node: &PsqlFromClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlFromClauseFields { from_token, items } = node.as_fields();

        // A from-item's own JOINs always hard-break (one per line, see
        // `FormatPsqlJoinClauseList`), and a hard break anywhere inside a
        // group forces that whole group to expand -- so wrapping a single
        // item in the same group as its joins would push the item itself
        // onto its own indented line too. That's only a real "does this
        // comma list of sources wrap" question with more than one item.
        if items.len() <= 1 {
            write!(f, [from_token.format(), space(), items.format()])
        } else {
            write!(
                f,
                [group(&format_args![
                    from_token.format(),
                    soft_line_indent_or_space(&items.format())
                ])]
            )
        }
    }
}
