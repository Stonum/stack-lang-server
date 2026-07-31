use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::NeedsParentheses;
use psql_syntax::PsqlParenthesizedExpression;
use psql_syntax::PsqlParenthesizedExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlParenthesizedExpression;
impl FormatNodeRule<PsqlParenthesizedExpression> for FormatPsqlParenthesizedExpression {
    fn fmt_fields(
        &self,
        node: &PsqlParenthesizedExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        // Only ever reached for a `PsqlParenthesizedExpression` the
        // preprocessing pass (`syntax_rewriter.rs`) deliberately kept --
        // one with a syntax error (bogus inner expression, skipped
        // trivia). Every "normal" parenthesized expression is stripped
        // from the tree before formatting and, if still needed, is
        // re-inserted by the inner expression's own `NeedsParentheses`
        // impl instead.
        let PsqlParenthesizedExpressionFields {
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

    fn needs_parentheses(&self, item: &PsqlParenthesizedExpression) -> bool {
        item.needs_parentheses()
    }
}
