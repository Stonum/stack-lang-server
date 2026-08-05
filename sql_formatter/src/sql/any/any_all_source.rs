//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlAnyAllSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlAnyAllSource;
impl FormatRule<AnySqlAnyAllSource> for FormatAnySqlAnyAllSource {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlAnyAllSource, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlAnyAllSource::SqlParenthesizedExpression(node) => node.format().fmt(f),
            AnySqlAnyAllSource::SqlSubqueryExpression(node) => node.format().fmt(f),
        }
    }
}
