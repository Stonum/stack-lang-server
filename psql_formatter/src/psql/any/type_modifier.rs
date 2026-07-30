//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlTypeModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlTypeModifier;
impl FormatRule<AnyPsqlTypeModifier> for FormatAnyPsqlTypeModifier {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlTypeModifier, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlTypeModifier::PsqlPrecisionModifier(node) => node.format().fmt(f),
            AnyPsqlTypeModifier::PsqlTimeZoneModifier(node) => node.format().fmt(f),
            AnyPsqlTypeModifier::PsqlVaryingModifier(node) => node.format().fmt(f),
        }
    }
}
