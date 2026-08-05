use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlStringLiteralExpression;
use sql_syntax::SqlStringLiteralExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlStringLiteralExpression;
impl FormatNodeRule<SqlStringLiteralExpression> for FormatSqlStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &SqlStringLiteralExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlStringLiteralExpressionFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
