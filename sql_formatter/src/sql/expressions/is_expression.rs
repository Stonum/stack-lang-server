use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::NeedsParentheses;
use sql_syntax::SqlIsExpression;
use sql_syntax::SqlIsExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlIsExpression;
impl FormatNodeRule<SqlIsExpression> for FormatSqlIsExpression {
    fn fmt_fields(&self, node: &SqlIsExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlIsExpressionFields {
            expression,
            is_token,
            not_token,
            value,
        } = node.as_fields();

        write!(f, [expression.format(), space(), is_token.format()])?;
        if let Some(not_token) = not_token {
            write!(f, [space(), not_token.format()])?;
        }
        write!(f, [space(), value.format()])
    }

    fn needs_parentheses(&self, item: &SqlIsExpression) -> bool {
        item.needs_parentheses()
    }
}
