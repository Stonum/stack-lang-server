use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::NeedsParentheses;
use sql_syntax::SqlIsDistinctFromExpression;
use sql_syntax::SqlIsDistinctFromExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlIsDistinctFromExpression;
impl FormatNodeRule<SqlIsDistinctFromExpression> for FormatSqlIsDistinctFromExpression {
    fn fmt_fields(
        &self,
        node: &SqlIsDistinctFromExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlIsDistinctFromExpressionFields {
            left,
            is_token,
            not_token,
            distinct_token,
            from_token,
            right,
        } = node.as_fields();

        write!(f, [left.format(), space(), is_token.format()])?;
        if let Some(not_token) = not_token {
            write!(f, [space(), not_token.format()])?;
        }
        write!(
            f,
            [
                space(),
                distinct_token.format(),
                space(),
                from_token.format(),
                space(),
                right.format()
            ]
        )
    }

    fn needs_parentheses(&self, item: &SqlIsDistinctFromExpression) -> bool {
        item.needs_parentheses()
    }
}
