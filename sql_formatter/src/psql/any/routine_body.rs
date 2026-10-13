//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlRoutineBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlRoutineBody;
impl FormatRule<AnyPsqlRoutineBody> for FormatAnyPsqlRoutineBody {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlRoutineBody, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlRoutineBody::SqlStringLiteralExpression(node) => node.format().fmt(f),
            AnyPsqlRoutineBody::PsqlPlBody(node) => node.format().fmt(f),
        }
    }
}
