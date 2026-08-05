use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCaseWhenClause;
use sql_syntax::SqlCaseWhenClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCaseWhenClause;
impl FormatNodeRule<SqlCaseWhenClause> for FormatSqlCaseWhenClause {
    fn fmt_fields(&self, node: &SqlCaseWhenClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCaseWhenClauseFields {
            when_token,
            condition,
            then_token,
            result,
        } = node.as_fields();

        write!(
            f,
            [
                when_token.format(),
                space(),
                condition.format(),
                space(),
                then_token.format(),
                space(),
                result.format(),
            ]
        )
    }
}
