use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlWindowFunctionExpression;
use sql_syntax::SqlWindowFunctionExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWindowFunctionExpression;
impl FormatNodeRule<SqlWindowFunctionExpression> for FormatSqlWindowFunctionExpression {
    fn fmt_fields(
        &self,
        node: &SqlWindowFunctionExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlWindowFunctionExpressionFields {
            call,
            over_token,
            window,
        } = node.as_fields();

        write!(
            f,
            [
                call.format(),
                space(),
                over_token.format(),
                space(),
                window.format()
            ]
        )
    }
}
