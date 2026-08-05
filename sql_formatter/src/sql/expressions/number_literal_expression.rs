use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlNumberLiteralExpression;
use sql_syntax::SqlNumberLiteralExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlNumberLiteralExpression;
impl FormatNodeRule<SqlNumberLiteralExpression> for FormatSqlNumberLiteralExpression {
    fn fmt_fields(
        &self,
        node: &SqlNumberLiteralExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlNumberLiteralExpressionFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
