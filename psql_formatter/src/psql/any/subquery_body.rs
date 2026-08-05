//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlSubqueryBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlSubqueryBody;
impl FormatRule<AnyPsqlSubqueryBody> for FormatAnyPsqlSubqueryBody {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlSubqueryBody, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlSubqueryBody::PsqlSelectStatement(node) => node.format().fmt(f),
            AnyPsqlSubqueryBody::PsqlValuesClause(node) => node.format().fmt(f),
        }
    }
}
