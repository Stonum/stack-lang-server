//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlFromExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlFromExpression;
impl FormatRule<AnySqlFromExpression> for FormatAnySqlFromExpression {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlFromExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlFromExpression::SqlFunctionBinding(node) => node.format().fmt(f),
            AnySqlFromExpression::SqlParenthesizedJoinBinding(node) => node.format().fmt(f),
            AnySqlFromExpression::SqlSubqueryBinding(node) => node.format().fmt(f),
            AnySqlFromExpression::SqlTableBinding(node) => node.format().fmt(f),
        }
    }
}
