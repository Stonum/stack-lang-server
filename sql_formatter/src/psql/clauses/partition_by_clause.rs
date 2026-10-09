use crate::prelude::*;
use crate::utils::{is_simple_expression, write_bracketed_fill_list};
use biome_formatter::write;
use sql_syntax::AnySqlExpression;
use sql_syntax::PsqlPartitionByClause;
use sql_syntax::PsqlPartitionByClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPartitionByClause;
impl FormatNodeRule<PsqlPartitionByClause> for FormatPsqlPartitionByClause {
    fn fmt_fields(&self, node: &PsqlPartitionByClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPartitionByClauseFields {
            partition_by_token,
            strategy,
            l_paren_token,
            keys,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                partition_by_token.format(),
                space(),
                strategy.format(),
                space()
            ]
        )?;
        write_bracketed_fill_list(
            l_paren_token,
            &keys,
            r_paren_token,
            |expr: &AnySqlExpression| !is_simple_expression(expr, 0),
            f,
        )
    }
}
