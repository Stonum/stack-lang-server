use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::NeedsParentheses;
use psql_syntax::PsqlIsNullExpression;
use psql_syntax::PsqlIsNullExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlIsNullExpression;
impl FormatNodeRule<PsqlIsNullExpression> for FormatPsqlIsNullExpression {
    fn fmt_fields(&self, node: &PsqlIsNullExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlIsNullExpressionFields {
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

    fn needs_parentheses(&self, item: &PsqlIsNullExpression) -> bool {
        item.needs_parentheses()
    }
}
