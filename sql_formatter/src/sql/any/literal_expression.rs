//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlLiteralExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlLiteralExpression;
impl FormatRule<AnySqlLiteralExpression> for FormatAnySqlLiteralExpression {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlLiteralExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlLiteralExpression::SqlBooleanLiteralExpression(node) => node.format().fmt(f),
            AnySqlLiteralExpression::SqlNullLiteralExpression(node) => node.format().fmt(f),
            AnySqlLiteralExpression::SqlNumberLiteralExpression(node) => node.format().fmt(f),
            AnySqlLiteralExpression::SqlStringLiteralExpression(node) => node.format().fmt(f),
        }
    }
}
