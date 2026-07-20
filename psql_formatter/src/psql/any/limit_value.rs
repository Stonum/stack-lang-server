//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlLimitValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlLimitValue;
impl FormatRule<AnyPsqlLimitValue> for FormatAnyPsqlLimitValue {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlLimitValue, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlLimitValue::PsqlNumberLiteralExpression(node) => node.format().fmt(f),
            AnyPsqlLimitValue::PsqlParameterExpression(node) => node.format().fmt(f),
        }
    }
}
