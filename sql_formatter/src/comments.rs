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
        let comment = match handle_hole_comment(comment) {
            CommentPlacement::Default(comment) => comment,
            placed => return placed,
        };
        let comment = match handle_create_function_body_comment(comment) {
            CommentPlacement::Default(comment) => comment,
            placed => return placed,
        };
        let comment = match handle_end_of_statement_comment(comment) {
            CommentPlacement::Default(comment) => comment,
            placed => return placed,
        };
        handle_sibling_boundary_comment(comment)
    }
}

/// A hole placeholder of dynamic SQL (see `embedded_sql.rs`) stays between
/// the same two tokens: it sticks to the node it touches.
fn handle_hole_comment(comment: DecoratedComment<SqlLanguage>) -> CommentPlacement<SqlLanguage> {
    if !crate::embedded_sql::is_hole_comment(comment.piece().text()) {
        return CommentPlacement::Default(comment);
    }

    let piece = comment.piece().as_piece();
    let token = piece.token();
    let next = if token.text_trimmed_range().start() >= piece.text_range().end() {
        Some(token)
    } else {
        token.next_token()
    };
    let previous = next.as_ref().and_then(|next| next.prev_token());

    if let Some(following) = comment.following_node()
        && following.first_token() == next
    {
        let following = following.clone();
        return CommentPlacement::leading(following, comment);
    }
    if let Some(preceding) = comment.preceding_node()
        && preceding.last_token() == previous
    {
        let preceding = preceding.clone();
        return CommentPlacement::trailing(preceding, comment);
    }
    CommentPlacement::Default(comment)
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

/// A comment on the same line as the end of a statement or declaration
/// (`x int; -- note`) stays its trailing comment. By default it would lead
/// the next statement and move to its own line -- before `begin` or after
/// it, depending on the pass. When the preceding node is a container (a
/// `DECLARE` section, a branch's statement list) the comment trails its last
/// item instead.
fn handle_end_of_statement_comment(
    comment: DecoratedComment<SqlLanguage>,
) -> CommentPlacement<SqlLanguage> {
    if comment.lines_before() > 0 {
        return CommentPlacement::Default(comment);
    }
    let Some(mut node) = comment.preceding_node().cloned() else {
        return CommentPlacement::Default(comment);
    };

    loop {
        let in_line_list = node.parent().is_some_and(|parent| {
            matches!(
                parent.kind(),
                SqlSyntaxKind::PSQL_PL_STATEMENT_LIST
                    | SqlSyntaxKind::PSQL_PL_DECLARATION_LIST
                    | SqlSyntaxKind::SQL_STATEMENT_LIST
            )
        });
        if in_line_list {
            return CommentPlacement::trailing(node, comment);
        }
        // Descend only into a child that ends where this node does, so the
        // comment really follows that child's last token.
        let Some(last) = node
            .last_child()
            .filter(|last| last.text_trimmed_range().end() == node.text_trimmed_range().end())
        else {
            return CommentPlacement::Default(comment);
        };
        node = last;
    }
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
