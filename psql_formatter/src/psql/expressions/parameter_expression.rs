use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlParameterExpression;
use psql_syntax::PsqlParameterExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlParameterExpression;
impl FormatNodeRule<PsqlParameterExpression> for FormatPsqlParameterExpression {
    fn fmt_fields(
        &self,
        node: &PsqlParameterExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlParameterExpressionFields { colon_token, name } = node.as_fields();

        write!(f, [colon_token.format(), name.format()])
    }
}
