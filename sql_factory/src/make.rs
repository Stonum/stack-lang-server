use std::fmt::Display;

use biome_rowan::TriviaPiece;
use sql_syntax::{AnySqlExpression, SqlParenthesizedExpression, SqlSyntaxKind, SqlSyntaxToken};

pub use super::generated::node_factory::*;

/// Create a new identifier token with no attached trivia
pub fn ident(text: &str) -> SqlSyntaxToken {
    SqlSyntaxToken::new_detached(SqlSyntaxKind::IDENT, text, [], [])
}

/// Create a new string literal token with no attached trivia
pub fn sql_string_literal(text: &str) -> SqlSyntaxToken {
    SqlSyntaxToken::new_detached(
        SqlSyntaxKind::SQL_STRING_LITERAL,
        &format!("'{text}'"),
        [],
        [],
    )
}

/// Create a new string literal token with no attached trivia
pub fn sql_number_literal<N>(text: N) -> SqlSyntaxToken
where
    N: Display + Copy,
{
    SqlSyntaxToken::new_detached(SqlSyntaxKind::SQL_NUMBER_LITERAL, &text.to_string(), [], [])
}

/// Create a new token with the specified syntax kind and no attached trivia
pub fn token(kind: SqlSyntaxKind) -> SqlSyntaxToken {
    if let Some(text) = kind.to_string() {
        SqlSyntaxToken::new_detached(kind, text, [], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// Create a new token with the specified syntax kind, and a whitespace trivia
/// piece on both the leading and trailing positions
pub fn token_decorated_with_space(kind: SqlSyntaxKind) -> SqlSyntaxToken {
    if let Some(text) = kind.to_string() {
        SqlSyntaxToken::new_detached(
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
pub fn eof() -> SqlSyntaxToken {
    SqlSyntaxToken::new_detached(SqlSyntaxKind::EOF, "", [], [])
}

/// Wrap `expr` in a new parenthesized expression
pub fn parenthesized(expr: impl Into<AnySqlExpression>) -> SqlParenthesizedExpression {
    sql_parenthesized_expression(
        token(SqlSyntaxKind::L_PAREN),
        expr.into(),
        token(SqlSyntaxKind::R_PAREN),
    )
}
