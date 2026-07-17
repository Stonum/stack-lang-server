//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlName;
impl FormatRule<AnyPsqlName> for FormatAnyPsqlName {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlName, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlName::PsqlName(node) => node.format().fmt(f),
            AnyPsqlName::PsqlTildeName(node) => node.format().fmt(f),
        }
    }
}
