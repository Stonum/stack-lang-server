//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlConflictTarget;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlConflictTarget;
impl FormatRule<AnyPsqlConflictTarget> for FormatAnyPsqlConflictTarget {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlConflictTarget, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlConflictTarget::PsqlColumnList(node) => node.format().fmt(f),
            AnyPsqlConflictTarget::PsqlOnConstraintClause(node) => node.format().fmt(f),
        }
    }
}
