//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlPartitionBound;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlPartitionBound;
impl FormatRule<AnyPsqlPartitionBound> for FormatAnyPsqlPartitionBound {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlPartitionBound, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlPartitionBound::PsqlPartitionDefaultBound(node) => node.format().fmt(f),
            AnyPsqlPartitionBound::PsqlPartitionForValuesBound(node) => node.format().fmt(f),
        }
    }
}
