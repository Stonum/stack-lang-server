//! psql supports parenthesizing expressions, e.g. `(a + b) * c`.
//! Parenthesizing an expression can be desired to change operator
//! precedence or to ease readability.
//!
//! This formatter is opinionated about which parentheses to keep or where
//! to insert parentheses: it removes parentheses that aren't necessary to
//! keep the original semantics, and inserts parentheses around nodes where
//! that's needed to preserve semantics (or, in a few cases, for
//! readability, e.g. mixing `and`/`or` without an explicit grouping).
//!
//! The [NeedsParentheses] trait forms the foundation of this. Its main
//! method, [NeedsParentheses::needs_parentheses], implements the rules for
//! when a node requires parentheses given its current position in the tree
//! (i.e. its parent).
//!
//! The challenge of formatting parenthesized nodes is that a tree with
//! parentheses and a tree without parentheses (that have the same
//! semantics) must result in the same output -- formatting `(a + 3) + 5`
//! must yield the same result as `a + 3 + 5` or `a + (3 + 5)`, even though
//! these trees differ in how many `PsqlParenthesizedExpression` nodes they
//! contain. This module only implements the *decision* half of that
//! (`needs_parentheses`); the other half -- stripping redundant
//! `PsqlParenthesizedExpression` nodes from the tree before formatting, so
//! both examples above produce the identical tree shape -- is a separate,
//! not-yet-built preprocessing pass.

mod expression;

/// Node that may be parenthesized to ensure it forms valid syntax or to
/// improve readability.
pub trait NeedsParentheses: biome_rowan::AstNode<Language = crate::PsqlLanguage> {
    /// Returns `true` if this node requires parentheses to form valid
    /// syntax or improve readability.
    ///
    /// Returns `false` if the parentheses can be omitted safely without
    /// changing the semantics.
    fn needs_parentheses(&self) -> bool;
}
