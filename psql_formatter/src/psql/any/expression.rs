//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlExpression;
impl FormatRule<AnyPsqlExpression> for FormatAnyPsqlExpression {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlExpression::AnyPsqlLiteralExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlArrayExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlArraySubscriptExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlBetweenExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlBinaryExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlCallExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlCaseExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlCastExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlCastFunctionExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlColReference(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlInExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlIsNullExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlLikeExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlLogicalExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlName(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlParameterExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlParenthesizedExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlStar(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlSubqueryExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlTableColReference(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlUnaryExpression(node) => node.format().fmt(f),
            AnyPsqlExpression::PsqlWindowFunctionExpression(node) => node.format().fmt(f),
        }
    }
}
