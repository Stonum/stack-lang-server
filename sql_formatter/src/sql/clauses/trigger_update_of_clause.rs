use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTriggerUpdateOfClause;
use sql_syntax::SqlTriggerUpdateOfClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTriggerUpdateOfClause;
impl FormatNodeRule<SqlTriggerUpdateOfClause> for FormatSqlTriggerUpdateOfClause {
    fn fmt_fields(
        &self,
        node: &SqlTriggerUpdateOfClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlTriggerUpdateOfClauseFields { of_token, columns } = node.as_fields();

        write!(f, [of_token.format(), space(), group(&columns.format())])
    }
}
