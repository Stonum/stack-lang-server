//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlReturnsType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlReturnsType;
impl FormatRule<AnySqlReturnsType> for FormatAnySqlReturnsType {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlReturnsType, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlReturnsType::SqlReturnsSetofClause(node) => node.format().fmt(f),
            AnySqlReturnsType::SqlReturnsTableClause(node) => node.format().fmt(f),
            AnySqlReturnsType::SqlReturnsTriggerClause(node) => node.format().fmt(f),
            AnySqlReturnsType::SqlTypeName(node) => node.format().fmt(f),
        }
    }
}
