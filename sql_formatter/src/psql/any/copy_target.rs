//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlCopyTarget;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlCopyTarget;
impl FormatRule<AnyPsqlCopyTarget> for FormatAnyPsqlCopyTarget {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlCopyTarget, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlCopyTarget::PsqlCopyFile(node) => node.format().fmt(f),
            AnyPsqlCopyTarget::PsqlCopyStdio(node) => node.format().fmt(f),
        }
    }
}
