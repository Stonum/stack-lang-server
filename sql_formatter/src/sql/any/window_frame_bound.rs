//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlWindowFrameBound;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlWindowFrameBound;
impl FormatRule<AnySqlWindowFrameBound> for FormatAnySqlWindowFrameBound {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlWindowFrameBound, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlWindowFrameBound::SqlWindowFrameCurrentRowBound(node) => node.format().fmt(f),
            AnySqlWindowFrameBound::SqlWindowFrameOffsetBound(node) => node.format().fmt(f),
            AnySqlWindowFrameBound::SqlWindowFrameUnboundedBound(node) => node.format().fmt(f),
        }
    }
}
