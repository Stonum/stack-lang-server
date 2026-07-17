use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlFromClause;
use psql_syntax::PsqlFromClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFromClause;
impl FormatNodeRule<PsqlFromClause> for FormatPsqlFromClause {
    fn fmt_fields(&self, node: &PsqlFromClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlFromClauseFields { from_token, items } = node.as_fields();

        write!(f, [from_token.format(), space(), items.format()])
    }
}
