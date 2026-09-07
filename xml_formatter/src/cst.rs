use super::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use super::{AsFormat, IntoFormat, XmlFormatContext};
use xml_syntax::{XmlSyntaxNode, map_syntax_node};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatXmlSyntaxNode;

impl biome_formatter::FormatRule<XmlSyntaxNode> for FormatXmlSyntaxNode {
    type Context = XmlFormatContext;

    fn fmt(&self, node: &XmlSyntaxNode, f: &mut XmlFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<XmlFormatContext> for XmlSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, XmlSyntaxNode, FormatXmlSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatXmlSyntaxNode)
    }
}

impl IntoFormat<XmlFormatContext> for XmlSyntaxNode {
    type Format = FormatOwnedWithRule<XmlSyntaxNode, FormatXmlSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatXmlSyntaxNode)
    }
}
