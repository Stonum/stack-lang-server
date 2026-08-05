use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::NeedsParentheses;
use sql_syntax::SqlUnaryExpression;
use sql_syntax::SqlUnaryExpressionFields;
use sql_syntax::T;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlUnaryExpression;
impl FormatNodeRule<SqlUnaryExpression> for FormatSqlUnaryExpression {
    fn fmt_fields(&self, node: &SqlUnaryExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlUnaryExpressionFields {
            operator_token,
            expression,
        } = node.as_fields();

        // `not` is a keyword and needs a separating space; `-`/`+` print
        // directly against their operand (`-x`, not `- x`). The one case
        // that could visually merge into `--` (a line comment) or `++` --
        // a same-sign operand nested directly underneath -- is handled by
        // `NeedsParentheses` wrapping that nested operand in parens
        // instead, so no extra spacing logic is needed here for that case.
        let is_not = operator_token
            .as_ref()
            .is_ok_and(|token| token.kind() == T![not]);

        write!(f, [operator_token.format()])?;
        if is_not {
            write!(f, [space()])?;
        }
        write!(f, [expression.format()])
    }

    fn needs_parentheses(&self, item: &SqlUnaryExpression) -> bool {
        item.needs_parentheses()
    }
}
