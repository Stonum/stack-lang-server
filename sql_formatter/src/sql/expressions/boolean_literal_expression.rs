use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlBooleanLiteralExpression;
use sql_syntax::SqlBooleanLiteralExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBooleanLiteralExpression;
impl FormatNodeRule<SqlBooleanLiteralExpression> for FormatSqlBooleanLiteralExpression {
    fn fmt_fields(
        &self,
        node: &SqlBooleanLiteralExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlBooleanLiteralExpressionFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
