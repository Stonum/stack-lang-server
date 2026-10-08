//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlTableConstraint;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlTableConstraint;
impl FormatRule<AnySqlTableConstraint> for FormatAnySqlTableConstraint {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlTableConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlTableConstraint::SqlCheckConstraint(node) => node.format().fmt(f),
            AnySqlTableConstraint::SqlForeignKeyTableConstraint(node) => node.format().fmt(f),
            AnySqlTableConstraint::SqlPrimaryKeyTableConstraint(node) => node.format().fmt(f),
            AnySqlTableConstraint::SqlUniqueTableConstraint(node) => node.format().fmt(f),
        }
    }
}
