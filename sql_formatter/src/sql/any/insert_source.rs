//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlInsertSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlInsertSource;
impl FormatRule<AnySqlInsertSource> for FormatAnySqlInsertSource {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlInsertSource, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlInsertSource::SqlSelectStatement(node) => node.format().fmt(f),
            AnySqlInsertSource::SqlSubqueryExpression(node) => node.format().fmt(f),
            AnySqlInsertSource::SqlValuesClause(node) => node.format().fmt(f),
        }
    }
}
