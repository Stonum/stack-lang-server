//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlTriggerTarget;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlTriggerTarget;
impl FormatRule<AnySqlTriggerTarget> for FormatAnySqlTriggerTarget {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlTriggerTarget, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlTriggerTarget::SqlAllTriggersTarget(node) => node.format().fmt(f),
            AnySqlTriggerTarget::SqlName(node) => node.format().fmt(f),
        }
    }
}
