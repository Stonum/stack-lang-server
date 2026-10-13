use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlFieldSelectionExpression;
use sql_syntax::PsqlFieldSelectionExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFieldSelectionExpression;
impl FormatNodeRule<PsqlFieldSelectionExpression> for FormatPsqlFieldSelectionExpression {
    fn fmt_fields(
        &self,
        node: &PsqlFieldSelectionExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlFieldSelectionExpressionFields {
            expression,
            dot_token,
            field,
        } = node.as_fields();

        write!(f, [expression.format(), dot_token.format(), field.format()])
    }
}
