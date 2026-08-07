use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTriggerReferencingClause;
use sql_syntax::PsqlTriggerReferencingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerReferencingClause;
impl FormatNodeRule<PsqlTriggerReferencingClause> for FormatPsqlTriggerReferencingClause {
    fn fmt_fields(
        &self,
        node: &PsqlTriggerReferencingClause,
        f: &mut SqlFormatter,
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
