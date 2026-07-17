//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlFromExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlFromExpression;
impl FormatRule<AnyPsqlFromExpression> for FormatAnyPsqlFromExpression {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlFromExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlFromExpression::PsqlFunctionBinding(node) => node.format().fmt(f),
            AnyPsqlFromExpression::PsqlSubqueryBinding(node) => node.format().fmt(f),
            AnyPsqlFromExpression::PsqlTableBinding(node) => node.format().fmt(f),
        }
    }
}
