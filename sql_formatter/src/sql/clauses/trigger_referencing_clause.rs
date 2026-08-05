use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTriggerReferencingClause;
use sql_syntax::SqlTriggerReferencingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTriggerReferencingClause;
impl FormatNodeRule<SqlTriggerReferencingClause> for FormatSqlTriggerReferencingClause {
    fn fmt_fields(
        &self,
        node: &SqlTriggerReferencingClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlTriggerReferencingClauseFields {
            referencing_token,
            items,
        } = node.as_fields();

        write!(
            f,
            [referencing_token.format(), space(), group(&items.format())]
        )
    }
}
