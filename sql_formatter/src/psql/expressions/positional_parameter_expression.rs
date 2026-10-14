use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPositionalParameterExpression;
use sql_syntax::PsqlPositionalParameterExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPositionalParameterExpression;
impl FormatNodeRule<PsqlPositionalParameterExpression> for FormatPsqlPositionalParameterExpression {
    fn fmt_fields(
        &self,
        node: &PsqlPositionalParameterExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlPositionalParameterExpressionFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
