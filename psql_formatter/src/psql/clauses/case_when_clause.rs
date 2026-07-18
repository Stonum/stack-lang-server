use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlCaseWhenClause;
use psql_syntax::PsqlCaseWhenClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCaseWhenClause;
impl FormatNodeRule<PsqlCaseWhenClause> for FormatPsqlCaseWhenClause {
    fn fmt_fields(&self, node: &PsqlCaseWhenClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlCaseWhenClauseFields {
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
