//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlSetItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlSetItem;
impl FormatRule<AnySqlSetItem> for FormatAnySqlSetItem {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlSetItem, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlSetItem::SqlSetItem(node) => node.format().fmt(f),
            AnySqlSetItem::PsqlMultiColumnSetItem(node) => node.format().fmt(f),
        }
    }
}
