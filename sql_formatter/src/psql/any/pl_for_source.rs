//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlPlForSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlPlForSource;
impl FormatRule<AnyPsqlPlForSource> for FormatAnyPsqlPlForSource {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlPlForSource, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlPlForSource::AnySqlStatement(node) => node.format().fmt(f),
            AnyPsqlPlForSource::SqlSubqueryExpression(node) => node.format().fmt(f),
            AnyPsqlPlForSource::PsqlPlForExecuteSource(node) => node.format().fmt(f),
            AnyPsqlPlForSource::PsqlPlForRangeSource(node) => node.format().fmt(f),
        }
    }
}
