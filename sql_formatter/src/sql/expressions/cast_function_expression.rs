use crate::prelude::*;
use biome_formatter::{format_args, write};
use sql_syntax::SqlCastFunctionExpression;
use sql_syntax::SqlCastFunctionExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCastFunctionExpression;
impl FormatNodeRule<SqlCastFunctionExpression> for FormatSqlCastFunctionExpression {
    fn fmt_fields(
        &self,
        node: &SqlCastFunctionExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlCastFunctionExpressionFields {
            cast_token,
            l_paren_token,
            expression,
            as_token,
            ty,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                cast_token.format(),
                l_paren_token.format(),
                group(&soft_block_indent(&format_args![
                    expression.format(),
                    soft_line_break_or_space(),
                    as_token.format(),
                    space(),
                    ty.format()
                ])),
                r_paren_token.format(),
            ]
        )
    }
}
