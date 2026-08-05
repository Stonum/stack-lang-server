use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use biome_formatter::write;
use sql_syntax::SqlDistinctOnClause;
use sql_syntax::SqlDistinctOnClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDistinctOnClause;
impl FormatNodeRule<SqlDistinctOnClause> for FormatSqlDistinctOnClause {
    fn fmt_fields(&self, node: &SqlDistinctOnClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDistinctOnClauseFields {
            on_token,
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(f, [on_token.format(), space()])?;
        write_bracketed_fill_list(l_paren_token, &items, r_paren_token, f)
    }
}
