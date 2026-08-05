use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlParameterExpression;
use sql_syntax::SqlParameterExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlParameterExpression;
impl FormatNodeRule<SqlParameterExpression> for FormatSqlParameterExpression {
    fn fmt_fields(&self, node: &SqlParameterExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlParameterExpressionFields { colon_token, name } = node.as_fields();

        write!(f, [colon_token.format(), name.format()])
    }
}
