use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlArraySubscriptExpression;
use sql_syntax::SqlArraySubscriptExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlArraySubscriptExpression;
impl FormatNodeRule<SqlArraySubscriptExpression> for FormatSqlArraySubscriptExpression {
    fn fmt_fields(
        &self,
        node: &SqlArraySubscriptExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlArraySubscriptExpressionFields {
            expression,
            l_brack_token,
            index,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                expression.format(),
                l_brack_token.format(),
                index.format(),
                r_brack_token.format(),
            ]
        )
    }
}
