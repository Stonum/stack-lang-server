//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlPlDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlPlDeclaration;
impl FormatRule<AnyPsqlPlDeclaration> for FormatAnyPsqlPlDeclaration {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlPlDeclaration, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlPlDeclaration::PsqlPlCursorDeclaration(node) => node.format().fmt(f),
            AnyPsqlPlDeclaration::PsqlPlRepeatedDeclare(node) => node.format().fmt(f),
            AnyPsqlPlDeclaration::PsqlPlVariableDeclaration(node) => node.format().fmt(f),
        }
    }
}
