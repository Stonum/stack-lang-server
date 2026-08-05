//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlTypeArraySuffix;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlTypeArraySuffix;
impl FormatRule<AnySqlTypeArraySuffix> for FormatAnySqlTypeArraySuffix {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlTypeArraySuffix, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlTypeArraySuffix::SqlTildeArraySuffix(node) => node.format().fmt(f),
            AnySqlTypeArraySuffix::SqlTypeArraySuffix(node) => node.format().fmt(f),
        }
    }
}
