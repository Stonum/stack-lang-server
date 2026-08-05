//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlSelectItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlSelectItem;
impl FormatRule<AnySqlSelectItem> for FormatAnySqlSelectItem {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlSelectItem, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlSelectItem::SqlSelectExpression(node) => node.format().fmt(f),
            AnySqlSelectItem::SqlStar(node) => node.format().fmt(f),
            AnySqlSelectItem::SqlTableStar(node) => node.format().fmt(f),
        }
    }
}
