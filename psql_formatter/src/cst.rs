use super::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use super::{AsFormat, IntoFormat, PsqlFormatContext};
use psql_syntax::{PsqlSyntaxNode, map_syntax_node};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatPsqlSyntaxNode;

impl biome_formatter::FormatRule<PsqlSyntaxNode> for FormatPsqlSyntaxNode {
    type Context = PsqlFormatContext;

    fn fmt(&self, node: &PsqlSyntaxNode, f: &mut PsqlFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<PsqlFormatContext> for PsqlSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, PsqlSyntaxNode, FormatPsqlSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatPsqlSyntaxNode)
    }
}

impl IntoFormat<PsqlFormatContext> for PsqlSyntaxNode {
    type Format = FormatOwnedWithRule<PsqlSyntaxNode, FormatPsqlSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatPsqlSyntaxNode)
    }
}
