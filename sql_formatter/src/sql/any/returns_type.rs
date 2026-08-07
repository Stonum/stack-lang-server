//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlReturnsType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlReturnsType;
impl FormatRule<AnySqlReturnsType> for FormatAnySqlReturnsType {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlReturnsType, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlReturnsType::SqlTypeName(node) => node.format().fmt(f),
            AnySqlReturnsType::PsqlReturnsSetofClause(node) => node.format().fmt(f),
            AnySqlReturnsType::PsqlReturnsTableClause(node) => node.format().fmt(f),
            AnySqlReturnsType::PsqlReturnsTriggerClause(node) => node.format().fmt(f),
        }
    }
}
