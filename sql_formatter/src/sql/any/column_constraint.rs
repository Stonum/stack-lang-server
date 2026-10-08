//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlColumnConstraint;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlColumnConstraint;
impl FormatRule<AnySqlColumnConstraint> for FormatAnySqlColumnConstraint {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlColumnConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlColumnConstraint::SqlDefaultConstraint(node) => node.format().fmt(f),
            AnySqlColumnConstraint::SqlNotNullConstraint(node) => node.format().fmt(f),
            AnySqlColumnConstraint::SqlNullConstraint(node) => node.format().fmt(f),
        }
    }
}
