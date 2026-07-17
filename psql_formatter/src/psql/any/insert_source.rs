//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlInsertSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlInsertSource;
impl FormatRule<AnyPsqlInsertSource> for FormatAnyPsqlInsertSource {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlInsertSource, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlInsertSource::PsqlInsertValues(node) => node.format().fmt(f),
            AnyPsqlInsertSource::PsqlSelectStatement(node) => node.format().fmt(f),
        }
    }
}
