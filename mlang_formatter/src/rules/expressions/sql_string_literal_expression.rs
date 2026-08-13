use crate::prelude::*;

use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{
    MSqlConcatenationExpression, MSqlLongStringLiteralExpression, MSqlStringLiteralExpression,
};

// Verbatim placeholders -- real formatting (try_format_embedded_sql /
// ConcatenatedQuery) lands in a later step of the same plan.

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
        format_verbatim_node(node.syntax()).fmt(f)
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
        format_verbatim_node(node.syntax()).fmt(f)
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
