//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlTypeArraySuffix;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlTypeArraySuffix;
impl FormatRule<AnyPsqlTypeArraySuffix> for FormatAnyPsqlTypeArraySuffix {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlTypeArraySuffix, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlTypeArraySuffix::PsqlTildeArraySuffix(node) => node.format().fmt(f),
            AnyPsqlTypeArraySuffix::PsqlTypeArraySuffix(node) => node.format().fmt(f),
        }
    }
}
