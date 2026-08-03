//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlSelectQuantifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlSelectQuantifier;
impl FormatRule<AnyPsqlSelectQuantifier> for FormatAnyPsqlSelectQuantifier {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlSelectQuantifier, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlSelectQuantifier::PsqlSelectAllQuantifier(node) => node.format().fmt(f),
            AnyPsqlSelectQuantifier::PsqlSelectDistinctQuantifier(node) => node.format().fmt(f),
        }
    }
}
