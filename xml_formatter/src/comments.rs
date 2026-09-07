use super::prelude::*;
use xml_syntax::XmlLanguage;

use biome_formatter::{
    comments::{CommentKind, CommentStyle, Comments, SourceComment},
    write,
};
use biome_rowan::SyntaxTriviaPieceComments;

pub type XmlComments = Comments<XmlLanguage>;

#[derive(Default)]
pub struct FormatXmlLeadingComment;

impl FormatRule<SourceComment<XmlLanguage>> for FormatXmlLeadingComment {
    type Context = XmlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<XmlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

/// XML comments (`<!-- ... -->`) are real CST nodes ([xml_syntax::XmlComment]),
/// not lexer trivia, so `Comments::from_node` never finds any and the methods
/// below are effectively unreachable. The impl still has to exist for the
/// `CstFormatContext` bound.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct XmlCommentStyle;

impl CommentStyle for XmlCommentStyle {
    type Language = XmlLanguage;

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<XmlLanguage>) -> CommentKind {
        CommentKind::Block
    }
}
