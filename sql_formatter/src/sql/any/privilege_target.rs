//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlPrivilegeTarget;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlPrivilegeTarget;
impl FormatRule<AnySqlPrivilegeTarget> for FormatAnySqlPrivilegeTarget {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlPrivilegeTarget, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlPrivilegeTarget::SqlPrivilegeAllInSchema(node) => node.format().fmt(f),
            AnySqlPrivilegeTarget::SqlPrivilegeObjects(node) => node.format().fmt(f),
        }
    }
}
