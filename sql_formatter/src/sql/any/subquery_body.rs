//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlSubqueryBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlSubqueryBody;
impl FormatRule<AnySqlSubqueryBody> for FormatAnySqlSubqueryBody {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlSubqueryBody, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlSubqueryBody::SqlSelectStatement(node) => node.format().fmt(f),
            AnySqlSubqueryBody::SqlValuesClause(node) => node.format().fmt(f),
        }
    }
}
