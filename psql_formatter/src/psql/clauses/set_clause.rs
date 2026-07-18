use crate::prelude::*;
use biome_formatter::{format_args, write};
use psql_syntax::PsqlSetClause;
use psql_syntax::PsqlSetClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSetClause;
impl FormatNodeRule<PsqlSetClause> for FormatPsqlSetClause {
    fn fmt_fields(&self, node: &PsqlSetClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSetClauseFields { set_token, items } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                set_token.format(),
                soft_line_indent_or_space(&items.format())
            ])]
        )
    }
}
