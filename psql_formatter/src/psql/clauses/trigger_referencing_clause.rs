use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTriggerReferencingClause;
use psql_syntax::PsqlTriggerReferencingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerReferencingClause;
impl FormatNodeRule<PsqlTriggerReferencingClause> for FormatPsqlTriggerReferencingClause {
    fn fmt_fields(
        &self,
        node: &PsqlTriggerReferencingClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlTriggerReferencingClauseFields {
            referencing_token,
            items,
        } = node.as_fields();

        write!(
            f,
            [referencing_token.format(), space(), group(&items.format())]
        )
    }
}
