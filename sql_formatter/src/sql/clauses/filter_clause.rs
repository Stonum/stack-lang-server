use crate::prelude::*;
use biome_formatter::{format_args, write};
use sql_syntax::SqlFilterClause;
use sql_syntax::SqlFilterClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFilterClause;
impl FormatNodeRule<SqlFilterClause> for FormatSqlFilterClause {
    fn fmt_fields(&self, node: &SqlFilterClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlFilterClauseFields {
            filter_token,
            l_paren_token,
            where_token,
            condition,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                filter_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&format_args![
                    where_token.format(),
                    space(),
                    condition.format(),
                ])),
                r_paren_token.format(),
            ]
        )
    }
}
