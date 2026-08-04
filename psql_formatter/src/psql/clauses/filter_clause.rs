use crate::prelude::*;
use biome_formatter::{format_args, write};
use psql_syntax::PsqlFilterClause;
use psql_syntax::PsqlFilterClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFilterClause;
impl FormatNodeRule<PsqlFilterClause> for FormatPsqlFilterClause {
    fn fmt_fields(&self, node: &PsqlFilterClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlFilterClauseFields {
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
