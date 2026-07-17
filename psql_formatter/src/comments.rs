use super::prelude::*;
use psql_syntax::PsqlLanguage;

use biome_formatter::{
    comments::{CommentKind, CommentStyle, Comments, SourceComment},
    write,
};
use biome_rowan::SyntaxTriviaPieceComments;

pub type PsqlComments = Comments<PsqlLanguage>;

#[derive(Default)]
pub struct FormatPsqlLeadingComment;

impl FormatRule<SourceComment<PsqlLanguage>> for FormatPsqlLeadingComment {
    type Context = PsqlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<PsqlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

/// Unlike mlang (which only ever lexes line comments and hardcodes
/// `CommentKind::Line`), SQL has both `--` line comments and `/* ... */`
/// block comments, so `get_comment_kind` needs a real implementation.
/// `is_suppression`/`place_comment` are left at their trait defaults --
/// no SQL-specific placement heuristics have been needed so far.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct PsqlCommentStyle;

impl CommentStyle for PsqlCommentStyle {
    type Language = PsqlLanguage;

    fn get_comment_kind(comment: &SyntaxTriviaPieceComments<PsqlLanguage>) -> CommentKind {
        if comment.text().starts_with("/*") {
            if comment.text().contains('\n') {
                CommentKind::Block
            } else {
                CommentKind::InlineBlock
            }
        } else {
            CommentKind::Line
        }
    }
}
