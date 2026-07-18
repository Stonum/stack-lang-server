use crate::prelude::*;
use biome_formatter::{format_args, write};
use psql_syntax::PsqlReturningClause;
use psql_syntax::PsqlReturningClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturningClause;
impl FormatNodeRule<PsqlReturningClause> for FormatPsqlReturningClause {
    fn fmt_fields(&self, node: &PsqlReturningClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlReturningClauseFields {
            returning_token,
            items,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                returning_token.format(),
                soft_line_indent_or_space(&items.format())
            ])]
        )
    }
}
