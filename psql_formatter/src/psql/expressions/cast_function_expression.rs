use crate::prelude::*;
use biome_formatter::{format_args, write};
use psql_syntax::PsqlCastFunctionExpression;
use psql_syntax::PsqlCastFunctionExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCastFunctionExpression;
impl FormatNodeRule<PsqlCastFunctionExpression> for FormatPsqlCastFunctionExpression {
    fn fmt_fields(
        &self,
        node: &PsqlCastFunctionExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlCastFunctionExpressionFields {
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
