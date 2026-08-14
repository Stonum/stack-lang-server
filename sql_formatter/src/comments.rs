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
/// `is_suppression` is left at its trait default; `place_comment` chains
/// [handle_create_function_body_comment] and
/// [handle_sibling_boundary_comment].
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
        let comment = match handle_create_function_body_comment(comment) {
            CommentPlacement::Default(comment) => comment,
            placed => return placed,
        };
        handle_sibling_boundary_comment(comment)
    }
}

/// A comment between two bare-token-separated clauses of `CREATE FUNCTION`
/// (e.g. between the parameter list and `returns ...`) has the same
/// instability as [handle_sibling_boundary_comment] below, but
/// `preceding_node`/`following_node` don't share an immediate parent here
/// (the parameter list nests one level deeper), so that check misses it.
fn handle_create_function_body_comment(
    comment: DecoratedComment<SqlLanguage>,
) -> CommentPlacement<SqlLanguage> {
    if comment.enclosing_node().kind() != SqlSyntaxKind::PSQL_CREATE_FUNCTION_STATEMENT {
        return CommentPlacement::Default(comment);
    }

    let Some(following) = comment.following_node() else {
        return CommentPlacement::Default(comment);
    };

    CommentPlacement::leading(following.clone(), comment)
}

/// A comment between two sibling nodes (same immediate parent) with no node
/// of its own to anchor to -- e.g. between two `and`/`or` operands, two
/// `union`-ed branches, or two items of a comma-separated list (`select`/
/// `group_by`/`order_by`/call arguments/`in (...)`). Default placement
/// (leading vs. trailing) depends on whether the comment shares a line with
/// the token before or after it, which flips depending on whether an
/// enclosing group/fill happened to collapse onto one line -- unstable
/// across a format pass, since that collapse decision can itself change.
/// Forcing it to always be a leading comment of the following sibling makes
/// placement depend only on tree structure.
fn handle_sibling_boundary_comment(
    comment: DecoratedComment<SqlLanguage>,
) -> CommentPlacement<SqlLanguage> {
    let (Some(preceding), Some(following)) = (comment.preceding_node(), comment.following_node())
    else {
        return CommentPlacement::Default(comment);
    };

    if preceding.parent() != following.parent() {
        return CommentPlacement::Default(comment);
    }

    let following = following.clone();
    CommentPlacement::leading(following, comment)
}
