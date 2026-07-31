use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::NeedsParentheses;
use psql_syntax::PsqlBetweenExpression;
use psql_syntax::PsqlBetweenExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBetweenExpression;
impl FormatNodeRule<PsqlBetweenExpression> for FormatPsqlBetweenExpression {
    fn fmt_fields(&self, node: &PsqlBetweenExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlBetweenExpressionFields {
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

    fn needs_parentheses(&self, item: &PsqlBetweenExpression) -> bool {
        item.needs_parentheses()
    }
}
