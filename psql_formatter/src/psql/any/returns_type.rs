//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlReturnsType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlReturnsType;
impl FormatRule<AnyPsqlReturnsType> for FormatAnyPsqlReturnsType {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlReturnsType, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlReturnsType::PsqlReturnsTableClause(node) => node.format().fmt(f),
            AnyPsqlReturnsType::PsqlReturnsTriggerClause(node) => node.format().fmt(f),
            AnyPsqlReturnsType::PsqlTypeName(node) => node.format().fmt(f),
        }
    }
}
