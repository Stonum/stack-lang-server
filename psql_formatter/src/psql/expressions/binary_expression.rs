use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::NeedsParentheses;
use psql_syntax::PsqlBinaryExpression;
use psql_syntax::PsqlBinaryExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBinaryExpression;
impl FormatNodeRule<PsqlBinaryExpression> for FormatPsqlBinaryExpression {
    fn fmt_fields(&self, node: &PsqlBinaryExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlBinaryExpressionFields {
            left,
            operator_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                operator_token.format(),
                space(),
                right.format(),
            ]
        )
    }

    fn needs_parentheses(&self, item: &PsqlBinaryExpression) -> bool {
        item.needs_parentheses()
    }
}
