use super::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use super::{AsFormat, IntoFormat, SqlFormatContext};
use sql_syntax::{SqlSyntaxNode, map_syntax_node};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatSqlSyntaxNode;

impl biome_formatter::FormatRule<SqlSyntaxNode> for FormatSqlSyntaxNode {
    type Context = SqlFormatContext;

    fn fmt(&self, node: &SqlSyntaxNode, f: &mut SqlFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<SqlFormatContext> for SqlSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, SqlSyntaxNode, FormatSqlSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatSqlSyntaxNode)
    }
}

impl IntoFormat<SqlFormatContext> for SqlSyntaxNode {
    type Format = FormatOwnedWithRule<SqlSyntaxNode, FormatSqlSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatSqlSyntaxNode)
    }
}
