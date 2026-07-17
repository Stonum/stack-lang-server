use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlNumberLiteralExpression;
use psql_syntax::PsqlNumberLiteralExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlNumberLiteralExpression;
impl FormatNodeRule<PsqlNumberLiteralExpression> for FormatPsqlNumberLiteralExpression {
    fn fmt_fields(
        &self,
        node: &PsqlNumberLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlNumberLiteralExpressionFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
