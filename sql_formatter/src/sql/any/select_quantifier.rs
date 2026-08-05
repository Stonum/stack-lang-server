//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlSelectQuantifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlSelectQuantifier;
impl FormatRule<AnySqlSelectQuantifier> for FormatAnySqlSelectQuantifier {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlSelectQuantifier, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlSelectQuantifier::SqlSelectAllQuantifier(node) => node.format().fmt(f),
            AnySqlSelectQuantifier::SqlSelectDistinctQuantifier(node) => node.format().fmt(f),
        }
    }
}
