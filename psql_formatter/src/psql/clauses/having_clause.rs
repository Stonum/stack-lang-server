use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlHavingClause;
use psql_syntax::PsqlHavingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlHavingClause;
impl FormatNodeRule<PsqlHavingClause> for FormatPsqlHavingClause {
    fn fmt_fields(&self, node: &PsqlHavingClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlHavingClauseFields {
            having_token,
            condition,
        } = node.as_fields();

        write!(f, [having_token.format(), space(), condition.format()])
    }
}
