//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlExpression;
impl FormatRule<AnySqlExpression> for FormatAnySqlExpression {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlExpression::AnySqlLiteralExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlAnyAllExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlBetweenExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlBinaryExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlCallExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlCaseExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlCastFunctionExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlColReference(node) => node.format().fmt(f),
            AnySqlExpression::SqlExistsExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlInExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlIsNullExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlLikeExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlLogicalExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlName(node) => node.format().fmt(f),
            AnySqlExpression::SqlParameterExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlParenthesizedExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlStar(node) => node.format().fmt(f),
            AnySqlExpression::SqlSubqueryExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlTableColReference(node) => node.format().fmt(f),
            AnySqlExpression::SqlTableStar(node) => node.format().fmt(f),
            AnySqlExpression::SqlUnaryExpression(node) => node.format().fmt(f),
            AnySqlExpression::SqlWindowFunctionExpression(node) => node.format().fmt(f),
            AnySqlExpression::PsqlArrayExpression(node) => node.format().fmt(f),
            AnySqlExpression::PsqlArraySubscriptExpression(node) => node.format().fmt(f),
            AnySqlExpression::PsqlCastExpression(node) => node.format().fmt(f),
            AnySqlExpression::PsqlIntervalExpression(node) => node.format().fmt(f),
            AnySqlExpression::PsqlSubstringExpression(node) => node.format().fmt(f),
            AnySqlExpression::PsqlTildeArrayExpression(node) => node.format().fmt(f),
        }
    }
}
