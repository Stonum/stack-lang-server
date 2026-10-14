//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlLockingWaitPolicy;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlLockingWaitPolicy;
impl FormatRule<AnyPsqlLockingWaitPolicy> for FormatAnyPsqlLockingWaitPolicy {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlLockingWaitPolicy, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlLockingWaitPolicy::PsqlNowaitPolicy(node) => node.format().fmt(f),
            AnyPsqlLockingWaitPolicy::PsqlSkipLockedPolicy(node) => node.format().fmt(f),
        }
    }
}
