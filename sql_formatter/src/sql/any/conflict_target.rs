//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlConflictTarget;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlConflictTarget;
impl FormatRule<AnySqlConflictTarget> for FormatAnySqlConflictTarget {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlConflictTarget, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlConflictTarget::SqlColumnList(node) => node.format().fmt(f),
            AnySqlConflictTarget::PsqlOnConstraintClause(node) => node.format().fmt(f),
        }
    }
}
