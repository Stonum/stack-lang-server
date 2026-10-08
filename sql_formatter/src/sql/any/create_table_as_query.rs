//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlCreateTableAsQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlCreateTableAsQuery;
impl FormatRule<AnySqlCreateTableAsQuery> for FormatAnySqlCreateTableAsQuery {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlCreateTableAsQuery, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlCreateTableAsQuery::SqlSelectStatement(node) => node.format().fmt(f),
            AnySqlCreateTableAsQuery::SqlSubqueryExpression(node) => node.format().fmt(f),
            AnySqlCreateTableAsQuery::PsqlTableQuery(node) => node.format().fmt(f),
        }
    }
}
