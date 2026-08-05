use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::NeedsParentheses;
use sql_syntax::SqlParenthesizedExpression;
use sql_syntax::SqlParenthesizedExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlParenthesizedExpression;
impl FormatNodeRule<SqlParenthesizedExpression> for FormatSqlParenthesizedExpression {
    fn fmt_fields(
        &self,
        node: &SqlParenthesizedExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        // Only ever reached for a `SqlParenthesizedExpression` the
        // preprocessing pass (`syntax_rewriter.rs`) deliberately kept --
        // one with a syntax error (bogus inner expression, skipped
        // trivia). Every "normal" parenthesized expression is stripped
        // from the tree before formatting and, if still needed, is
        // re-inserted by the inner expression's own `NeedsParentheses`
        // impl instead.
        let SqlParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                group(&soft_block_indent(&expression.format())),
                r_paren_token.format(),
            ]
        )
    }

    fn needs_parentheses(&self, item: &SqlParenthesizedExpression) -> bool {
        item.needs_parentheses()
    }
}
