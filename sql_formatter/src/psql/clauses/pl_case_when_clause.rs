use crate::prelude::*;
use crate::psql::lists::pl_statement_list::FormatPlBranchStatements;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use biome_formatter::write;
use sql_syntax::AnySqlExpression;
use sql_syntax::PsqlPlCaseWhenClause;
use sql_syntax::PsqlPlCaseWhenClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlCaseWhenClause;
impl FormatNodeRule<PsqlPlCaseWhenClause> for FormatPsqlPlCaseWhenClause {
    fn fmt_fields(&self, node: &PsqlPlCaseWhenClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlCaseWhenClauseFields {
            when_token,
            conditions,
            then_token,
            statements,
        } = node.as_fields();

        write!(
            f,
            [group(&format_once(|f| {
                write_wrapping_fill_clause(
                    when_token.format(),
                    &conditions,
                    |condition: &AnySqlExpression| !is_simple_expression(condition, 0),
                    f,
                )
            }))]
        )?;
        write!(
            f,
            [
                space(),
                then_token.format(),
                FormatPlBranchStatements::new(&statements)
            ]
        )
    }
}
