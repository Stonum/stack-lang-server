use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::NeedsParentheses;
use sql_syntax::SqlLikeExpression;
use sql_syntax::SqlLikeExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlLikeExpression;
impl FormatNodeRule<SqlLikeExpression> for FormatSqlLikeExpression {
    fn fmt_fields(&self, node: &SqlLikeExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlLikeExpressionFields {
            expression,
            not_token,
            operator_token,
            pattern,
        } = node.as_fields();

        write!(f, [expression.format(), space()])?;
        if let Some(not_token) = not_token {
            write!(f, [not_token.format(), space()])?;
        }
        write!(f, [operator_token.format(), space(), pattern.format()])
    }

    fn needs_parentheses(&self, item: &SqlLikeExpression) -> bool {
        item.needs_parentheses()
    }
}
