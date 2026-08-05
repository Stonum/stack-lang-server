//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlName;
impl FormatRule<AnySqlName> for FormatAnySqlName {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlName, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlName::SqlName(node) => node.format().fmt(f),
            AnySqlName::SqlTildeName(node) => node.format().fmt(f),
        }
    }
}
