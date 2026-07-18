use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlArraySubscriptExpression;
use psql_syntax::PsqlArraySubscriptExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlArraySubscriptExpression;
impl FormatNodeRule<PsqlArraySubscriptExpression> for FormatPsqlArraySubscriptExpression {
    fn fmt_fields(
        &self,
        node: &PsqlArraySubscriptExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlArraySubscriptExpressionFields {
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
