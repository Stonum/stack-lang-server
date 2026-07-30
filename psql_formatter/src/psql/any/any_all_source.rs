//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlAnyAllSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlAnyAllSource;
impl FormatRule<AnyPsqlAnyAllSource> for FormatAnyPsqlAnyAllSource {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlAnyAllSource, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlAnyAllSource::PsqlParenthesizedExpression(node) => node.format().fmt(f),
            AnyPsqlAnyAllSource::PsqlSubqueryExpression(node) => node.format().fmt(f),
        }
    }
}
