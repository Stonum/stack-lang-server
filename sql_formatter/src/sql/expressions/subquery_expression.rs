use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSubqueryExpression;
use sql_syntax::SqlSubqueryExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSubqueryExpression;
impl FormatNodeRule<SqlSubqueryExpression> for FormatSqlSubqueryExpression {
    fn fmt_fields(&self, node: &SqlSubqueryExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSubqueryExpressionFields {
            l_paren_token,
            query,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                group(&soft_block_indent(&query.format())),
                r_paren_token.format(),
            ]
        )
    }
}
