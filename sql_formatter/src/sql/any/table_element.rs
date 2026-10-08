//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlTableElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlTableElement;
impl FormatRule<AnySqlTableElement> for FormatAnySqlTableElement {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlTableElement, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlTableElement::SqlColumnDefinition(node) => node.format().fmt(f),
            AnySqlTableElement::SqlTableConstraint(node) => node.format().fmt(f),
        }
    }
}
