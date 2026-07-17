//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlLiteralExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlLiteralExpression;
impl FormatRule<AnyPsqlLiteralExpression> for FormatAnyPsqlLiteralExpression {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlLiteralExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlLiteralExpression::PsqlBooleanLiteralExpression(node) => node.format().fmt(f),
            AnyPsqlLiteralExpression::PsqlNullLiteralExpression(node) => node.format().fmt(f),
            AnyPsqlLiteralExpression::PsqlNumberLiteralExpression(node) => node.format().fmt(f),
            AnyPsqlLiteralExpression::PsqlStringLiteralExpression(node) => node.format().fmt(f),
        }
    }
}
