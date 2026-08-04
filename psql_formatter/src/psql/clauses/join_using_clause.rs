use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlJoinUsingClause;
use psql_syntax::PsqlJoinUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlJoinUsingClause;
impl FormatNodeRule<PsqlJoinUsingClause> for FormatPsqlJoinUsingClause {
    fn fmt_fields(&self, node: &PsqlJoinUsingClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlJoinUsingClauseFields {
            using_token,
            columns,
        } = node.as_fields();

        write!(f, [using_token.format(), space(), columns.format()])
    }
}
