//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlPlStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlPlStatement;
impl FormatRule<AnyPsqlPlStatement> for FormatAnyPsqlPlStatement {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlPlStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlPlStatement::AnySqlStatement(node) => node.format().fmt(f),
            AnyPsqlPlStatement::PsqlPlAssignmentStatement(node) => node.format().fmt(f),
            AnyPsqlPlStatement::PsqlPlBlock(node) => node.format().fmt(f),
            AnyPsqlPlStatement::PsqlPlExecuteStatement(node) => node.format().fmt(f),
            AnyPsqlPlStatement::PsqlPlIfStatement(node) => node.format().fmt(f),
            AnyPsqlPlStatement::PsqlPlPerformStatement(node) => node.format().fmt(f),
            AnyPsqlPlStatement::PsqlPlRaiseStatement(node) => node.format().fmt(f),
            AnyPsqlPlStatement::PsqlPlReturnQueryStatement(node) => node.format().fmt(f),
            AnyPsqlPlStatement::PsqlPlReturnStatement(node) => node.format().fmt(f),
        }
    }
}
