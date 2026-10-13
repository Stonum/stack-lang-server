use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::NeedsParentheses;
use sql_syntax::SqlPostfixNullExpression;
use sql_syntax::SqlPostfixNullExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPostfixNullExpression;
impl FormatNodeRule<SqlPostfixNullExpression> for FormatSqlPostfixNullExpression {
    fn fmt_fields(
        &self,
        node: &SqlPostfixNullExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlPostfixNullExpressionFields {
            expression,
            operator,
        } = node.as_fields();

        write!(f, [expression.format(), space(), operator.format()])
    }

    fn needs_parentheses(&self, item: &SqlPostfixNullExpression) -> bool {
        item.needs_parentheses()
    }
}
