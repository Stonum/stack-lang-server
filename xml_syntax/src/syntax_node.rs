use crate::{XmlRoot, XmlSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct XmlLanguage;

impl Language for XmlLanguage {
    type Kind = XmlSyntaxKind;
    type Root = XmlRoot;
}

pub type XmlSyntaxNode = biome_rowan::SyntaxNode<XmlLanguage>;
pub type XmlSyntaxToken = biome_rowan::SyntaxToken<XmlLanguage>;
pub type XmlSyntaxElement = biome_rowan::SyntaxElement<XmlLanguage>;
pub type XmlSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<XmlLanguage>;
pub type XmlSyntaxElementChildren = biome_rowan::SyntaxElementChildren<XmlLanguage>;
pub type XmlSyntaxList = biome_rowan::SyntaxList<XmlLanguage>;
pub type XmlSyntaxTrivia = biome_rowan::syntax::SyntaxTrivia<XmlLanguage>;
