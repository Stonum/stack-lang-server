use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTriggerUpdateOfClause;
use psql_syntax::PsqlTriggerUpdateOfClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerUpdateOfClause;
impl FormatNodeRule<PsqlTriggerUpdateOfClause> for FormatPsqlTriggerUpdateOfClause {
    fn fmt_fields(
        &self,
        node: &PsqlTriggerUpdateOfClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlTriggerUpdateOfClauseFields { of_token, columns } = node.as_fields();

        write!(f, [of_token.format(), space(), group(&columns.format())])
    }
}
