use crate::prelude::*;
use crate::utils::{ConcatenatedQuery, FormatSqlStringToken};

use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{
    AnyMExpression, MSqlConcatenationExpression, MSqlLongStringLiteralExpression,
    MSqlLongStringLiteralExpressionFields, MSqlStringLiteralExpression,
    MSqlStringLiteralExpressionFields,
};

// The parser only produces these kinds once the content already parsed as
// real SQL, so no selection/option is needed to decide -- unlike the plain
// string-literal rules.

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
        let expression = node.expression()?;
        let any_expression = AnyMExpression::from(expression.clone());

        // ConcatenatedQuery may still decline (e.g. a comment on a bypassed
        // piece); fall back to ordinary MBinaryExpression formatting.
        match ConcatenatedQuery::try_new(&any_expression, f) {
            Some(query) => query.fmt(f),
            None => expression.format().fmt(f),
        }
    }

    fn needs_parentheses(&self, item: &MSqlConcatenationExpression) -> bool {
        item.needs_parentheses()
    }
}
