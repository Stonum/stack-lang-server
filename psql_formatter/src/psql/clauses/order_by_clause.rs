use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlOrderByClause;
use psql_syntax::PsqlOrderByClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOrderByClause;
impl FormatNodeRule<PsqlOrderByClause> for FormatPsqlOrderByClause {
    fn fmt_fields(&self, node: &PsqlOrderByClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlOrderByClauseFields {
            order_by_token,
            items,
        } = node.as_fields();

        write!(f, [order_by_token.format(), space(), items.format()])
    }
}
