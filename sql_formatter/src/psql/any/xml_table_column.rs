//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnyPsqlXmlTableColumn;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlXmlTableColumn;
impl FormatRule<AnyPsqlXmlTableColumn> for FormatAnyPsqlXmlTableColumn {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnyPsqlXmlTableColumn, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlXmlTableColumn::PsqlXmlTableOrdinalityColumn(node) => node.format().fmt(f),
            AnyPsqlXmlTableColumn::PsqlXmlTableValueColumn(node) => node.format().fmt(f),
        }
    }
}
