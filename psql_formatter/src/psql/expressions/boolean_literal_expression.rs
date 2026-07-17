use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlBooleanLiteralExpression;
use psql_syntax::PsqlBooleanLiteralExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBooleanLiteralExpression;
impl FormatNodeRule<PsqlBooleanLiteralExpression> for FormatPsqlBooleanLiteralExpression {
    fn fmt_fields(
        &self,
        node: &PsqlBooleanLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlBooleanLiteralExpressionFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
