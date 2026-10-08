//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlColumnConstraint;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlColumnConstraint;
impl FormatRule<AnySqlColumnConstraint> for FormatAnySqlColumnConstraint {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlColumnConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlColumnConstraint::SqlCheckConstraint(node) => node.format().fmt(f),
            AnySqlColumnConstraint::SqlConstraintName(node) => node.format().fmt(f),
            AnySqlColumnConstraint::SqlDefaultConstraint(node) => node.format().fmt(f),
            AnySqlColumnConstraint::SqlNotNullConstraint(node) => node.format().fmt(f),
            AnySqlColumnConstraint::SqlNullConstraint(node) => node.format().fmt(f),
            AnySqlColumnConstraint::SqlPrimaryKeyConstraint(node) => node.format().fmt(f),
            AnySqlColumnConstraint::SqlReferencesConstraint(node) => node.format().fmt(f),
            AnySqlColumnConstraint::SqlUniqueConstraint(node) => node.format().fmt(f),
            AnySqlColumnConstraint::PsqlGeneratedColumnConstraint(node) => node.format().fmt(f),
            AnySqlColumnConstraint::PsqlIdentityConstraint(node) => node.format().fmt(f),
        }
    }
}
