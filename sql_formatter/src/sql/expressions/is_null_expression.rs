use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::NeedsParentheses;
use sql_syntax::SqlIsNullExpression;
use sql_syntax::SqlIsNullExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlIsNullExpression;
impl FormatNodeRule<SqlIsNullExpression> for FormatSqlIsNullExpression {
    fn fmt_fields(&self, node: &SqlIsNullExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlIsNullExpressionFields {
            expression,
            is_token,
            not_token,
            null_token,
        } = node.as_fields();

        write!(f, [expression.format(), space(), is_token.format()])?;
        if let Some(not_token) = not_token {
            write!(f, [space(), not_token.format()])?;
        }
        write!(f, [space(), null_token.format()])
    }

    fn needs_parentheses(&self, item: &SqlIsNullExpression) -> bool {
        item.needs_parentheses()
    }
}
