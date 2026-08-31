use crate::prelude::*;
use crate::utils::{is_simple_expression, write_bracketed_fill_list};
use biome_formatter::write;
use sql_syntax::AnySqlExpression;
use sql_syntax::PsqlTildeArrayExpression;
use sql_syntax::PsqlTildeArrayExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTildeArrayExpression;
impl FormatNodeRule<PsqlTildeArrayExpression> for FormatPsqlTildeArrayExpression {
    fn fmt_fields(
        &self,
        node: &PsqlTildeArrayExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlTildeArrayExpressionFields {
            array_token,
            open_tilde_token,
            l_brack_token,
            items,
            r_brack_token,
            close_tilde_token,
        } = node.as_fields();

        write!(f, [array_token.format(), open_tilde_token.format()])?;
        write_bracketed_fill_list(
            l_brack_token,
            &items,
            r_brack_token,
            |expr: &AnySqlExpression| !is_simple_expression(expr, 0),
            f,
        )?;
        write!(f, [close_tilde_token.format()])
    }
}
