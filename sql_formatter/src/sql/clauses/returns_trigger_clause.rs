use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlReturnsTriggerClause;
use sql_syntax::SqlReturnsTriggerClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlReturnsTriggerClause;
impl FormatNodeRule<SqlReturnsTriggerClause> for FormatSqlReturnsTriggerClause {
    fn fmt_fields(&self, node: &SqlReturnsTriggerClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlReturnsTriggerClauseFields { trigger_token } = node.as_fields();

        write!(f, [trigger_token.format()])
    }
}
