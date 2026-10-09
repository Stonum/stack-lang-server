//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlAlterTableAction;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlAlterTableAction;
impl FormatRule<AnySqlAlterTableAction> for FormatAnySqlAlterTableAction {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlAlterTableAction, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlAlterTableAction::SqlAddColumnAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::SqlAddConstraintAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::SqlAlterColumnAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::SqlAlterTriggerAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::SqlDropColumnAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::SqlDropConstraintAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::SqlRenameColumnAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::SqlRenameConstraintAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::SqlRenameTableAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::SqlSetSchemaAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::PsqlAttachPartitionAction(node) => node.format().fmt(f),
            AnySqlAlterTableAction::PsqlDetachPartitionAction(node) => node.format().fmt(f),
        }
    }
}
