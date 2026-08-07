use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use biome_formatter::write;
use sql_syntax::PsqlDistinctOnClause;
use sql_syntax::PsqlDistinctOnClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDistinctOnClause;
impl FormatNodeRule<PsqlDistinctOnClause> for FormatPsqlDistinctOnClause {
    fn fmt_fields(&self, node: &PsqlDistinctOnClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlDistinctOnClauseFields {
            on_token,
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(f, [on_token.format(), space()])?;
        write_bracketed_fill_list(l_paren_token, &items, r_paren_token, f)
    }
}
