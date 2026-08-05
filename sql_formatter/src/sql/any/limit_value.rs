//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlLimitValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlLimitValue;
impl FormatRule<AnySqlLimitValue> for FormatAnySqlLimitValue {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlLimitValue, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlLimitValue::SqlNumberLiteralExpression(node) => node.format().fmt(f),
            AnySqlLimitValue::SqlParameterExpression(node) => node.format().fmt(f),
        }
    }
}
