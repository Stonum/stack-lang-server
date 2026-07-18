use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlCastExpression;
use psql_syntax::PsqlCastExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCastExpression;
impl FormatNodeRule<PsqlCastExpression> for FormatPsqlCastExpression {
    fn fmt_fields(&self, node: &PsqlCastExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlCastExpressionFields {
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
