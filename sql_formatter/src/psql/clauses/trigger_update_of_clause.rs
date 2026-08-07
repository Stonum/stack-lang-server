use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTriggerUpdateOfClause;
use sql_syntax::PsqlTriggerUpdateOfClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerUpdateOfClause;
impl FormatNodeRule<PsqlTriggerUpdateOfClause> for FormatPsqlTriggerUpdateOfClause {
    fn fmt_fields(
        &self,
        node: &PsqlTriggerUpdateOfClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlTriggerUpdateOfClauseFields { of_token, columns } = node.as_fields();

        write!(f, [of_token.format(), space(), group(&columns.format())])
    }
}
