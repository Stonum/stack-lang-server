use super::prelude::*;
use sql_syntax::{SqlLanguage, SqlSyntaxKind};

use biome_formatter::{
    comments::{
        CommentKind, CommentPlacement, CommentStyle, Comments, DecoratedComment, SourceComment,
    },
    write,
};
use biome_rowan::SyntaxTriviaPieceComments;

pub type SqlComments = Comments<SqlLanguage>;

#[derive(Default)]
pub struct FormatSqlLeadingComment;

impl FormatRule<SourceComment<SqlLanguage>> for FormatSqlLeadingComment {
    type Context = SqlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<SqlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

/// Unlike mlang (which only ever lexes line comments and hardcodes
/// `CommentKind::Line`), SQL has both `--` line comments and `/* ... */`
/// block comments, so `get_comment_kind` needs a real implementation.
/// `is_suppression` is left at its trait default; `place_comment` has one
/// SQL-specific override (see [handle_create_function_body_comment]) --
/// no other placement heuristics have been needed so far.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct SqlCommentStyle;

impl CommentStyle for SqlCommentStyle {
    type Language = SqlLanguage;

    fn get_comment_kind(comment: &SyntaxTriviaPieceComments<SqlLanguage>) -> CommentKind {
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

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        handle_create_function_body_comment(comment)
    }
}

/// A comment between two bare-token-separated clauses of `CREATE FUNCTION`
/// (e.g. between the parameter list and `returns ...`, or between
/// `returns ...`/the trailing options and `as`) ends up as trivia on a bare
/// token (`)`, `as`, ...), with no node of its own to anchor to. Its
/// default placement (leading vs. trailing, and of which token) depends on
/// whether it happens to share a line with the token before or after it --
/// which flips depending on the *exact* whitespace/newline shape of
/// whatever text it's attached to. Since that shape itself changes across a
/// format pass (a hard line break the first pass inserts becomes the
/// context the second pass lexes from), the default placement isn't
/// stable, and comments can end up reordered, dropped onto the wrong
/// clause, or merged onto one line on the second pass.
///
/// Forcing the comment to always be a *leading* comment of whatever clause
/// follows it (its unambiguous `following_node`) makes the placement
/// depend only on tree structure, not on incidental whitespace -- fixing
/// the instability.
fn handle_create_function_body_comment(
    comment: DecoratedComment<SqlLanguage>,
) -> CommentPlacement<SqlLanguage> {
    if comment.enclosing_node().kind() != SqlSyntaxKind::SQL_CREATE_FUNCTION_STATEMENT {
        return CommentPlacement::Default(comment);
    }

    let Some(following) = comment.following_node() else {
        return CommentPlacement::Default(comment);
    };

    CommentPlacement::leading(following.clone(), comment)
}
