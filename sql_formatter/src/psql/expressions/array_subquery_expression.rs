use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlArraySubqueryExpression;
use sql_syntax::PsqlArraySubqueryExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlArraySubqueryExpression;
impl FormatNodeRule<PsqlArraySubqueryExpression> for FormatPsqlArraySubqueryExpression {
    fn fmt_fields(
        &self,
        node: &PsqlArraySubqueryExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlArraySubqueryExpressionFields { array_token, query } = node.as_fields();

        write!(f, [array_token.format(), query.format()])
    }
}
