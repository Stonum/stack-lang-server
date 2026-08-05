use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTriggerForEachClause;
use sql_syntax::SqlTriggerForEachClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTriggerForEachClause;
impl FormatNodeRule<SqlTriggerForEachClause> for FormatSqlTriggerForEachClause {
    fn fmt_fields(&self, node: &SqlTriggerForEachClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTriggerForEachClauseFields {
            for_token,
            each_token,
            granularity,
        } = node.as_fields();

        write!(
            f,
            [
                for_token.format(),
                space(),
                each_token.format(),
                space(),
                granularity.format()
            ]
        )
    }
}
