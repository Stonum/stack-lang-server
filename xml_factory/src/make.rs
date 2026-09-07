use biome_rowan::TriviaPiece;
use xml_syntax::{XmlSyntaxKind, XmlSyntaxToken};

pub use super::generated::node_factory::*;

/// Create a new identifier/name token with no attached trivia.
pub fn xml_literal(text: &str) -> XmlSyntaxToken {
    XmlSyntaxToken::new_detached(XmlSyntaxKind::XML_LITERAL, text, [], [])
}

/// Create a new double-quoted string literal token with no attached trivia.
pub fn xml_string_literal(text: &str) -> XmlSyntaxToken {
    XmlSyntaxToken::new_detached(
        XmlSyntaxKind::XML_STRING_LITERAL,
        &format!("\"{text}\""),
        [],
        [],
    )
}

/// Create a new token with the specified syntax kind and no attached trivia.
pub fn token(kind: XmlSyntaxKind) -> XmlSyntaxToken {
    if let Some(text) = kind.to_string() {
        XmlSyntaxToken::new_detached(kind, text, [], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// Create a new token with the specified syntax kind, and a whitespace trivia
/// piece on both the leading and trailing positions.
pub fn token_decorated_with_space(kind: XmlSyntaxKind) -> XmlSyntaxToken {
    if let Some(text) = kind.to_string() {
        XmlSyntaxToken::new_detached(
            kind,
            &format!(" {text} "),
            [TriviaPiece::whitespace(1)],
            [TriviaPiece::whitespace(1)],
        )
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// EOF token
pub fn eof() -> XmlSyntaxToken {
    XmlSyntaxToken::new_detached(XmlSyntaxKind::EOF, "", [], [])
}
