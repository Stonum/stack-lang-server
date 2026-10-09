//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlPartitionValues;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlPartitionValues;
impl FormatRule<AnyPsqlPartitionValues> for FormatAnyPsqlPartitionValues {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlPartitionValues, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlPartitionValues::PsqlPartitionHashValues(node) => node.format().fmt(f),
            AnyPsqlPartitionValues::PsqlPartitionInValues(node) => node.format().fmt(f),
            AnyPsqlPartitionValues::PsqlPartitionRangeValues(node) => node.format().fmt(f),
        }
    }
}
