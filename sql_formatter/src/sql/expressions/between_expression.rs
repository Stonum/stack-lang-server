use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::NeedsParentheses;
use sql_syntax::SqlBetweenExpression;
use sql_syntax::SqlBetweenExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBetweenExpression;
impl FormatNodeRule<SqlBetweenExpression> for FormatSqlBetweenExpression {
    fn fmt_fields(&self, node: &SqlBetweenExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlBetweenExpressionFields {
            expression,
            not_token,
            between_token,
            low,
            and_token,
            high,
        } = node.as_fields();

        write!(f, [expression.format(), space()])?;
        if let Some(not_token) = not_token {
            write!(f, [not_token.format(), space()])?;
        }
        write!(
            f,
            [
                between_token.format(),
                space(),
                low.format(),
                space(),
                and_token.format(),
                space(),
                high.format(),
            ]
        )
    }

    fn needs_parentheses(&self, item: &SqlBetweenExpression) -> bool {
        item.needs_parentheses()
    }
}
