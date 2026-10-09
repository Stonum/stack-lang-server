//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlAlterColumnOperation;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlAlterColumnOperation;
impl FormatRule<AnySqlAlterColumnOperation> for FormatAnySqlAlterColumnOperation {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlAlterColumnOperation, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlAlterColumnOperation::SqlAlterColumnType(node) => node.format().fmt(f),
            AnySqlAlterColumnOperation::SqlDropColumnDefault(node) => node.format().fmt(f),
            AnySqlAlterColumnOperation::SqlDropColumnNotNull(node) => node.format().fmt(f),
            AnySqlAlterColumnOperation::SqlSetColumnDefault(node) => node.format().fmt(f),
            AnySqlAlterColumnOperation::SqlSetColumnNotNull(node) => node.format().fmt(f),
        }
    }
}
