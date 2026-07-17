//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlStatement;
impl FormatRule<AnyPsqlStatement> for FormatAnyPsqlStatement {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlStatement::PsqlBogusStatement(node) => node.format().fmt(f),
            AnyPsqlStatement::PsqlDeleteStatement(node) => node.format().fmt(f),
            AnyPsqlStatement::PsqlEmptyStatement(node) => node.format().fmt(f),
            AnyPsqlStatement::PsqlInsertStatement(node) => node.format().fmt(f),
            AnyPsqlStatement::PsqlSelectStatement(node) => node.format().fmt(f),
            AnyPsqlStatement::PsqlUpdateStatement(node) => node.format().fmt(f),
        }
    }
}
