//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlConflictAction;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlConflictAction;
impl FormatRule<AnySqlConflictAction> for FormatAnySqlConflictAction {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlConflictAction, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlConflictAction::SqlDoNothingClause(node) => node.format().fmt(f),
            AnySqlConflictAction::SqlDoUpdateClause(node) => node.format().fmt(f),
        }
    }
}
