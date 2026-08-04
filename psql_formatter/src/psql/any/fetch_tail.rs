//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlFetchTail;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlFetchTail;
impl FormatRule<AnyPsqlFetchTail> for FormatAnyPsqlFetchTail {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlFetchTail, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlFetchTail::PsqlFetchOnlyTail(node) => node.format().fmt(f),
            AnyPsqlFetchTail::PsqlFetchWithTiesTail(node) => node.format().fmt(f),
        }
    }
}
