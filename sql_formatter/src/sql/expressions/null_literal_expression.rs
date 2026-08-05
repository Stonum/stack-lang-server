use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlNullLiteralExpression;
use sql_syntax::SqlNullLiteralExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlNullLiteralExpression;
impl FormatNodeRule<SqlNullLiteralExpression> for FormatSqlNullLiteralExpression {
    fn fmt_fields(
        &self,
        node: &SqlNullLiteralExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlNullLiteralExpressionFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
