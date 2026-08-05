use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::NeedsParentheses;
use sql_syntax::SqlBinaryExpression;
use sql_syntax::SqlBinaryExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBinaryExpression;
impl FormatNodeRule<SqlBinaryExpression> for FormatSqlBinaryExpression {
    fn fmt_fields(&self, node: &SqlBinaryExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlBinaryExpressionFields {
            left,
            operator_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                operator_token.format(),
                space(),
                right.format(),
            ]
        )
    }

    fn needs_parentheses(&self, item: &SqlBinaryExpression) -> bool {
        item.needs_parentheses()
    }
}
