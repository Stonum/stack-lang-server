//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlTypeModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlTypeModifier;
impl FormatRule<AnySqlTypeModifier> for FormatAnySqlTypeModifier {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlTypeModifier, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlTypeModifier::SqlPrecisionModifier(node) => node.format().fmt(f),
            AnySqlTypeModifier::SqlTimeZoneModifier(node) => node.format().fmt(f),
            AnySqlTypeModifier::SqlVaryingModifier(node) => node.format().fmt(f),
        }
    }
}
