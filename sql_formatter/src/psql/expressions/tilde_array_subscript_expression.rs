use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTildeArraySubscriptExpression;
use sql_syntax::PsqlTildeArraySubscriptExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTildeArraySubscriptExpression;
impl FormatNodeRule<PsqlTildeArraySubscriptExpression> for FormatPsqlTildeArraySubscriptExpression {
    fn fmt_fields(
        &self,
        node: &PsqlTildeArraySubscriptExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlTildeArraySubscriptExpressionFields {
            expression,
            open_tilde_token,
            l_brack_token,
            index,
            r_brack_token,
            close_tilde_token,
        } = node.as_fields();

        write!(
            f,
            [
                expression.format(),
                open_tilde_token.format(),
                l_brack_token.format(),
                index.format(),
                r_brack_token.format(),
                close_tilde_token.format()
            ]
        )
    }
}
