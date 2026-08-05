use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCastExpression;
use sql_syntax::SqlCastExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCastExpression;
impl FormatNodeRule<SqlCastExpression> for FormatSqlCastExpression {
    fn fmt_fields(&self, node: &SqlCastExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCastExpressionFields {
            expression,
            double_colon_token,
            ty,
        } = node.as_fields();

        write!(
            f,
            [
                expression.format(),
                double_colon_token.format(),
                ty.format()
            ]
        )
    }
}
