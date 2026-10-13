use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use sql_syntax::AnySqlExpression;
use sql_syntax::PsqlPlUsingClause;
use sql_syntax::PsqlPlUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlUsingClause;
impl FormatNodeRule<PsqlPlUsingClause> for FormatPsqlPlUsingClause {
    fn fmt_fields(&self, node: &PsqlPlUsingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlUsingClauseFields {
            using_token,
            arguments,
        } = node.as_fields();

        write_wrapping_fill_clause(
            using_token.format(),
            &arguments,
            |argument: &AnySqlExpression| !is_simple_expression(argument, 0),
            f,
        )
    }
}
