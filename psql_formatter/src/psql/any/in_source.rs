//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlInSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlInSource;
impl FormatRule<AnyPsqlInSource> for FormatAnyPsqlInSource {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlInSource, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlInSource::PsqlInValueList(node) => node.format().fmt(f),
            AnyPsqlInSource::PsqlSubqueryExpression(node) => node.format().fmt(f),
        }
    }
}
