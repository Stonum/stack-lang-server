use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlJoinUsingClause;
use sql_syntax::PsqlJoinUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlJoinUsingClause;
impl FormatNodeRule<PsqlJoinUsingClause> for FormatPsqlJoinUsingClause {
    fn fmt_fields(&self, node: &PsqlJoinUsingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlJoinUsingClauseFields {
            using_token,
            columns,
        } = node.as_fields();

        write!(f, [using_token.format(), space(), columns.format()])
    }
}
