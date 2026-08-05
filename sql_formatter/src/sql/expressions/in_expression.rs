use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::NeedsParentheses;
use sql_syntax::SqlInExpression;
use sql_syntax::SqlInExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlInExpression;
impl FormatNodeRule<SqlInExpression> for FormatSqlInExpression {
    fn fmt_fields(&self, node: &SqlInExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlInExpressionFields {
            expression,
            not_token,
            in_token,
            source,
        } = node.as_fields();

        write!(f, [expression.format(), space()])?;
        if let Some(not_token) = not_token {
            write!(f, [not_token.format(), space()])?;
        }
        write!(f, [in_token.format(), space(), source.format()])
    }

    fn needs_parentheses(&self, item: &SqlInExpression) -> bool {
        item.needs_parentheses()
    }
}
