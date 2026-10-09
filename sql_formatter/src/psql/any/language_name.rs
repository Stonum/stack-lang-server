//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlLanguageName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlLanguageName;
impl FormatRule<AnyPsqlLanguageName> for FormatAnyPsqlLanguageName {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlLanguageName, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlLanguageName::SqlName(node) => node.format().fmt(f),
            AnyPsqlLanguageName::SqlStringLiteralExpression(node) => node.format().fmt(f),
        }
    }
}
