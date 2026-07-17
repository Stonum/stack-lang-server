//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlSelectItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlSelectItem;
impl FormatRule<AnyPsqlSelectItem> for FormatAnyPsqlSelectItem {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlSelectItem, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlSelectItem::PsqlSelectExpression(node) => node.format().fmt(f),
            AnyPsqlSelectItem::PsqlStar(node) => node.format().fmt(f),
        }
    }
}
