//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlPlExceptionCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlPlExceptionCondition;
impl FormatRule<AnyPsqlPlExceptionCondition> for FormatAnyPsqlPlExceptionCondition {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlPlExceptionCondition, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlPlExceptionCondition::SqlName(node) => node.format().fmt(f),
            AnyPsqlPlExceptionCondition::PsqlPlSqlstateCondition(node) => node.format().fmt(f),
        }
    }
}
