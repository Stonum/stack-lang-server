use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlNullLiteralExpression;
use psql_syntax::PsqlNullLiteralExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlNullLiteralExpression;
impl FormatNodeRule<PsqlNullLiteralExpression> for FormatPsqlNullLiteralExpression {
    fn fmt_fields(
        &self,
        node: &PsqlNullLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlNullLiteralExpressionFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
