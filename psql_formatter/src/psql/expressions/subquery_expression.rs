use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlSubqueryExpression;
use psql_syntax::PsqlSubqueryExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSubqueryExpression;
impl FormatNodeRule<PsqlSubqueryExpression> for FormatPsqlSubqueryExpression {
    fn fmt_fields(&self, node: &PsqlSubqueryExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSubqueryExpressionFields {
            l_paren_token,
            query,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                block_indent(&query.format()),
                r_paren_token.format(),
            ]
        )
    }
}
