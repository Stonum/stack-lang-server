//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlConflictAction;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlConflictAction;
impl FormatRule<AnyPsqlConflictAction> for FormatAnyPsqlConflictAction {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlConflictAction, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlConflictAction::PsqlDoNothingClause(node) => node.format().fmt(f),
            AnyPsqlConflictAction::PsqlDoUpdateClause(node) => node.format().fmt(f),
        }
    }
}
