use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::NeedsParentheses;
use psql_syntax::PsqlLikeExpression;
use psql_syntax::PsqlLikeExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLikeExpression;
impl FormatNodeRule<PsqlLikeExpression> for FormatPsqlLikeExpression {
    fn fmt_fields(&self, node: &PsqlLikeExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlLikeExpressionFields {
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

    fn needs_parentheses(&self, item: &PsqlLikeExpression) -> bool {
        item.needs_parentheses()
    }
}
