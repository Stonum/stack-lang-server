use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPositionExpression;
use sql_syntax::PsqlPositionExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPositionExpression;
impl FormatNodeRule<PsqlPositionExpression> for FormatPsqlPositionExpression {
    fn fmt_fields(&self, node: &PsqlPositionExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPositionExpressionFields {
            name_token,
            l_paren_token,
            substring,
            in_token,
            string,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name_token.format(),
                l_paren_token.format(),
                substring.format(),
                space(),
                in_token.format(),
                space(),
                string.format(),
                r_paren_token.format()
            ]
        )
    }
}
