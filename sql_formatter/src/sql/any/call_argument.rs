//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlCallArgument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlCallArgument;
impl FormatRule<AnySqlCallArgument> for FormatAnySqlCallArgument {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlCallArgument, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlCallArgument::AnySqlExpression(node) => node.format().fmt(f),
            AnySqlCallArgument::PsqlNamedArgument(node) => node.format().fmt(f),
        }
    }
}
