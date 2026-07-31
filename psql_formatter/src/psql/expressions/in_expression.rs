use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::NeedsParentheses;
use psql_syntax::PsqlInExpression;
use psql_syntax::PsqlInExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlInExpression;
impl FormatNodeRule<PsqlInExpression> for FormatPsqlInExpression {
    fn fmt_fields(&self, node: &PsqlInExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlInExpressionFields {
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

    fn needs_parentheses(&self, item: &PsqlInExpression) -> bool {
        item.needs_parentheses()
    }
}
