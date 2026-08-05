//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlInSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlInSource;
impl FormatRule<AnySqlInSource> for FormatAnySqlInSource {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlInSource, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlInSource::SqlInValueList(node) => node.format().fmt(f),
            AnySqlInSource::SqlSubqueryExpression(node) => node.format().fmt(f),
        }
    }
}
