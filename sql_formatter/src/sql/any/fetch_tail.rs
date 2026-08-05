//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlFetchTail;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlFetchTail;
impl FormatRule<AnySqlFetchTail> for FormatAnySqlFetchTail {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlFetchTail, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlFetchTail::SqlFetchOnlyTail(node) => node.format().fmt(f),
            AnySqlFetchTail::SqlFetchWithTiesTail(node) => node.format().fmt(f),
        }
    }
}
