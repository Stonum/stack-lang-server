#[macro_use]
mod generated;
mod file_source;
mod parentheses;
mod syntax_node;

use biome_rowan::RawSyntaxKind;
pub use biome_rowan::{
    AstNode, Language, SendNode, SyntaxNode, TextLen, TextRange, TextSize, TokenAtOffset,
    TokenText, TriviaPieceKind, WalkEvent,
};

pub use crate::file_source::*;
use crate::generated::kind::SqlSyntaxKind::*;
pub use crate::generated::*;
pub use crate::parentheses::NeedsParentheses;
pub use crate::syntax_node::*;

impl From<u16> for SqlSyntaxKind {
    fn from(d: u16) -> SqlSyntaxKind {
        assert!(d <= (SqlSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SqlSyntaxKind>(d) }
    }
}

impl From<SqlSyntaxKind> for u16 {
    fn from(k: SqlSyntaxKind) -> u16 {
        k as u16
    }
}

impl SqlSyntaxKind {
    /// Returns `true` for any contextual (await) or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        (self as u16) <= (SqlSyntaxKind::FALSE_KW as u16)
            && (self as u16) >= (SqlSyntaxKind::INTEGER_KW as u16)
    }
}

impl biome_rowan::SyntaxKind for SqlSyntaxKind {
    const TOMBSTONE: Self = TOMBSTONE;
    const EOF: Self = EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            SQL_BOGUS
                | SQL_BOGUS_STATEMENT
                | SQL_BOGUS_PARAMETER
                | SQL_BOGUS_BINDING
                | SQL_BOGUS_MEMBER
                | SQL_BOGUS_EXPRESSION
                | SQL_BOGUS_ASSIGNMENT
        )
    }

    fn to_bogus(&self) -> SqlSyntaxKind {
        match self {
            kind if AnySqlExpression::can_cast(*kind) => SQL_BOGUS_EXPRESSION,
            _ => SQL_BOGUS,
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
        SqlRoot::can_cast(*self)
    }

    fn is_list(&self) -> bool {
        SqlSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(
            self,
            SqlSyntaxKind::NEWLINE | SqlSyntaxKind::WHITESPACE | SqlSyntaxKind::COMMENT
        )
    }

    fn to_string(&self) -> Option<&'static str> {
        SqlSyntaxKind::to_string(self)
    }
}

impl TryFrom<SqlSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: SqlSyntaxKind) -> Result<Self, Self::Error> {
        match value {
            SqlSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
            SqlSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
            SqlSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone, Hash)]
pub enum OperatorPrecedence {
    Comma = 0,
    Assignment = 2,
    Conditional = 3,
    LogicalOr = 5,
    LogicalAnd = 6,
    /// `not x` — binds looser than every other operator (including
    /// comparisons) but tighter than `and`/`or`.
    Not = 7,
    /// `x is [not] null` — a postfix operator, binds looser than
    /// comparisons but tighter than `not`/`and`/`or`.
    IsNull = 8,
    BitwiseOr = 9,
    BitwiseXor = 10,
    BitwiseAnd = 11,
    Equality = 12,
    Relational = 13,
    /// The shared precedence tier for the SQL "predicates" `[not] between`,
    /// `[not] in` and `[not] like`/`ilike` — binds tighter than comparisons
    /// but looser than arithmetic and the generic pattern-match operators
    /// already folded into [OperatorPrecedence::Relational].
    Predicate = 14,
    /// `||` (string concatenation) and `->`/`->>` (JSON field/text
    /// extraction) -- all share real Postgres's generic "any other
    /// operator" precedence: tighter than comparisons and the
    /// `between`/`in`/`like` predicates but looser than the arithmetic
    /// tiers.
    Concat = 15,
    Shift = 16,
    Additive = 17,
    Multiplicative = 18,
    Unary = 19,
    Update = 20,
    LeftHandSide = 22,
    Member = 23,
    Primary = 24,
    Group = 25,
}

impl OperatorPrecedence {
    /// Returns the operator with the lowest precedence
    pub fn lowest() -> Self {
        OperatorPrecedence::Comma
    }

    /// Returns the operator with the highest precedence
    #[allow(dead_code)]
    pub fn highest() -> Self {
        OperatorPrecedence::Primary
    }

    /// Returns `true` if this operator has right to left associativity
    pub fn is_right_to_left(&self) -> bool {
        matches!(
            self,
            OperatorPrecedence::Assignment
                | OperatorPrecedence::Conditional
                | OperatorPrecedence::Update
        )
    }

    /// Returns the precedence for a binary operator token or [None] if the token isn't a binary operator
    pub fn try_from_binary_operator(kind: SqlSyntaxKind) -> Option<OperatorPrecedence> {
        Some(match kind {
            T![or] => OperatorPrecedence::LogicalOr,
            T![and] => OperatorPrecedence::LogicalAnd,
            T![|] => OperatorPrecedence::BitwiseOr,
            T![^] => OperatorPrecedence::BitwiseXor,
            T![&] => OperatorPrecedence::BitwiseAnd,
            T![=] | T![!=] | T![<>] => OperatorPrecedence::Equality,
            T![<]
            | T![>]
            | T![<=]
            | T![>=]
            | T![~]
            | T![!~]
            | T![~*]
            | T![!~*]
            | T![~~]
            | T![!~~]
            | T![~~*]
            | T![!~~*] => OperatorPrecedence::Relational,
            T![||] | T![->] | T![->>] => OperatorPrecedence::Concat,
            T![<<] | T![>>] => OperatorPrecedence::Shift,
            T![+] | T![-] => OperatorPrecedence::Additive,
            T![*] | T![/] | T![%] => OperatorPrecedence::Multiplicative,
            _ => return None,
        })
    }

    pub const fn is_bitwise(&self) -> bool {
        matches!(
            self,
            OperatorPrecedence::BitwiseAnd
                | OperatorPrecedence::BitwiseOr
                | OperatorPrecedence::BitwiseXor
        )
    }

    pub const fn is_additive(&self) -> bool {
        matches!(self, OperatorPrecedence::Additive)
    }

    pub const fn is_equality(&self) -> bool {
        matches!(self, OperatorPrecedence::Equality)
    }

    pub const fn is_multiplicative(&self) -> bool {
        matches!(self, OperatorPrecedence::Multiplicative)
    }
}

pub fn inner_string_text(token: &SqlSyntaxToken) -> TokenText {
    let mut text = token.token_text_trimmed();
    if matches!(token.kind(), SqlSyntaxKind::SQL_STRING_LITERAL) {
        // remove string delimiters
        // SAFETY: string literal token have a delimiters at the start and the end of the string
        let range = TextRange::new(1.into(), text.len() - TextSize::from(1));
        text = text.slice(range);
    }
    text
}
