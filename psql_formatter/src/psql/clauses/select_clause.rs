use crate::prelude::*;
use biome_formatter::{format_args, write};
use psql_syntax::PsqlSelectClause;
use psql_syntax::PsqlSelectClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectClause;
impl FormatNodeRule<PsqlSelectClause> for FormatPsqlSelectClause {
    fn fmt_fields(&self, node: &PsqlSelectClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSelectClauseFields { select_token, list } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                select_token.format(),
                soft_line_indent_or_space(&list.format())
            ])]
        )
    }
}
