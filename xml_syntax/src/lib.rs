#[macro_use]
mod generated;
mod file_source;
mod syntax_node;

use biome_rowan::RawSyntaxKind;
pub use biome_rowan::{
    AstNode, Language, SendNode, SyntaxNode, TextLen, TextRange, TextSize, TokenAtOffset,
    TokenText, TriviaPieceKind, WalkEvent,
};

pub use crate::file_source::*;
use crate::generated::kind::XmlSyntaxKind::*;
pub use crate::generated::*;
pub use crate::syntax_node::*;

impl From<u16> for XmlSyntaxKind {
    fn from(d: u16) -> XmlSyntaxKind {
        assert!(d <= (XmlSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, XmlSyntaxKind>(d) }
    }
}

impl From<XmlSyntaxKind> for u16 {
    fn from(k: XmlSyntaxKind) -> u16 {
        k as u16
    }
}

impl biome_rowan::SyntaxKind for XmlSyntaxKind {
    const TOMBSTONE: Self = TOMBSTONE;
    const EOF: Self = EOF;

    fn is_bogus(&self) -> bool {
        matches!(self, XML_BOGUS | XML_BOGUS_ELEMENT | XML_BOGUS_ATTRIBUTE)
    }

    fn to_bogus(&self) -> XmlSyntaxKind {
        match self {
            kind if AnyXmlAttribute::can_cast(*kind) => XML_BOGUS_ATTRIBUTE,
            kind if AnyXmlElement::can_cast(*kind) => XML_BOGUS_ELEMENT,
            _ => XML_BOGUS,
        }
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        XmlRoot::can_cast(*self)
    }

    fn is_list(&self) -> bool {
        XmlSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(self, XmlSyntaxKind::NEWLINE | XmlSyntaxKind::WHITESPACE)
    }

    fn to_string(&self) -> Option<&'static str> {
        XmlSyntaxKind::to_string(self)
    }
}

impl TryFrom<XmlSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: XmlSyntaxKind) -> Result<Self, Self::Error> {
        match value {
            XmlSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
            XmlSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
            _ => Err(()),
        }
    }
}

/// The text of an attribute-value string token without its surrounding
/// quotes. Entity references (`&apos;`, `&quot;`, …) are **not** decoded.
pub fn inner_string_text(token: &XmlSyntaxToken) -> TokenText {
    let mut text = token.token_text_trimmed();
    if token.kind() == XmlSyntaxKind::XML_STRING_LITERAL && text.len() >= TextSize::from(2) {
        // A string literal token always has a delimiter at each end.
        let range = TextRange::new(1.into(), text.len() - TextSize::from(1));
        text = text.slice(range);
    }
    text
}
