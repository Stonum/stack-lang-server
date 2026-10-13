use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use sql_syntax::PsqlPlRaiseOption;
use sql_syntax::PsqlPlRaiseUsingClause;
use sql_syntax::PsqlPlRaiseUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlRaiseUsingClause;
impl FormatNodeRule<PsqlPlRaiseUsingClause> for FormatPsqlPlRaiseUsingClause {
    fn fmt_fields(&self, node: &PsqlPlRaiseUsingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlRaiseUsingClauseFields {
            using_token,
            options,
        } = node.as_fields();

        write_wrapping_fill_clause(
            using_token.format(),
            &options,
            |option: &PsqlPlRaiseOption| {
                option
                    .value()
                    .is_ok_and(|value| !is_simple_expression(&value, 0))
            },
            f,
        )
    }
}
