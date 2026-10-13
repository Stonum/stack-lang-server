//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlPlOpenSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlPlOpenSource;
impl FormatRule<AnyPsqlPlOpenSource> for FormatAnyPsqlPlOpenSource {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlPlOpenSource, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlPlOpenSource::AnySqlStatement(node) => node.format().fmt(f),
            AnyPsqlPlOpenSource::PsqlPlForExecuteSource(node) => node.format().fmt(f),
        }
    }
}
