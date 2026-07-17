use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlStringLiteralExpression;
use psql_syntax::PsqlStringLiteralExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlStringLiteralExpression;
impl FormatNodeRule<PsqlStringLiteralExpression> for FormatPsqlStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &PsqlStringLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlStringLiteralExpressionFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
