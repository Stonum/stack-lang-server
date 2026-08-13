use crate::prelude::*;
use crate::utils::FormatSqlStringToken;

use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{
    MSqlConcatenationExpression, MSqlLongStringLiteralExpression,
    MSqlLongStringLiteralExpressionFields, MSqlStringLiteralExpression,
    MSqlStringLiteralExpressionFields,
};

// The parser only produces these kinds once the token's content has already
// parsed as real SQL (see `mlang_parser::sql_literal_rewriter`), so
// `FormatSqlStringToken` always has real embedded SQL to format here --
// unlike the plain `MStringLiteralExpression`/`MLongStringLiteralExpression`
// rules, no `is_query_like_string`/selection option is needed to decide.
// `FormatSqlStringToken` still falls back to plain literal formatting on its
// own if parsing surprisingly fails (defensive, not expected to trigger).

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMSqlStringLiteralExpression;
impl_format_with_rule!(
    MSqlStringLiteralExpression,
    FormatMSqlStringLiteralExpression
);

impl FormatNodeRule<MSqlStringLiteralExpression> for FormatMSqlStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &MSqlStringLiteralExpression,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let MSqlStringLiteralExpressionFields { value_token } = node.as_fields();
        FormatSqlStringToken::new(&value_token?).fmt(f)
    }

    fn needs_parentheses(&self, item: &MSqlStringLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMSqlLongStringLiteralExpression;
impl_format_with_rule!(
    MSqlLongStringLiteralExpression,
    FormatMSqlLongStringLiteralExpression
);

impl FormatNodeRule<MSqlLongStringLiteralExpression> for FormatMSqlLongStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &MSqlLongStringLiteralExpression,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let MSqlLongStringLiteralExpressionFields { value_token } = node.as_fields();
        FormatSqlStringToken::new(&value_token?).fmt(f)
    }

    fn needs_parentheses(&self, item: &MSqlLongStringLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMSqlConcatenationExpression;
impl_format_with_rule!(
    MSqlConcatenationExpression,
    FormatMSqlConcatenationExpression
);

impl FormatNodeRule<MSqlConcatenationExpression> for FormatMSqlConcatenationExpression {
    fn fmt_fields(
        &self,
        node: &MSqlConcatenationExpression,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }

    fn needs_parentheses(&self, item: &MSqlConcatenationExpression) -> bool {
        item.needs_parentheses()
    }
}
