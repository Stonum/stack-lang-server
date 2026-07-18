use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlWindowFunctionExpression;
use psql_syntax::PsqlWindowFunctionExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWindowFunctionExpression;
impl FormatNodeRule<PsqlWindowFunctionExpression> for FormatPsqlWindowFunctionExpression {
    fn fmt_fields(
        &self,
        node: &PsqlWindowFunctionExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlWindowFunctionExpressionFields {
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
