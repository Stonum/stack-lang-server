//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlCopySource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlCopySource;
impl FormatRule<AnyPsqlCopySource> for FormatAnyPsqlCopySource {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlCopySource, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlCopySource::SqlSubqueryExpression(node) => node.format().fmt(f),
            AnyPsqlCopySource::PsqlCopyTableSource(node) => node.format().fmt(f),
        }
    }
}
