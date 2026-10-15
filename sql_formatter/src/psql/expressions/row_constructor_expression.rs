use crate::prelude::*;
use crate::utils::{is_simple_expression, write_bracketed_fill_list};
use sql_syntax::AnySqlExpression;
use sql_syntax::PsqlRowConstructorExpression;
use sql_syntax::PsqlRowConstructorExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlRowConstructorExpression;
impl FormatNodeRule<PsqlRowConstructorExpression> for FormatPsqlRowConstructorExpression {
    fn fmt_fields(
        &self,
        node: &PsqlRowConstructorExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlRowConstructorExpressionFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(
            l_paren_token,
            &items,
            r_paren_token,
            |expr: &AnySqlExpression| !is_simple_expression(expr, 0),
            f,
        )
    }
}
