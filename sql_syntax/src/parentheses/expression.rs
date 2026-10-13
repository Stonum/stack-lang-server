use biome_rowan::AstNode;

use crate::{
    AnySqlExpression, AnySqlLiteralExpression, OperatorPrecedence, SqlBetweenExpression,
    SqlBinaryExpression, SqlInExpression, SqlIsDistinctFromExpression, SqlIsExpression,
    SqlLikeExpression, SqlLogicalExpression, SqlParenthesizedExpression, SqlPostfixNullExpression,
    SqlSyntaxKind, SqlSyntaxNode, SqlUnaryExpression,
};

use super::NeedsParentheses;

impl NeedsParentheses for AnySqlExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::AnySqlLiteralExpression(expr) => expr.needs_parentheses(),
            Self::SqlBinaryExpression(expr) => expr.needs_parentheses(),
            Self::SqlLogicalExpression(expr) => expr.needs_parentheses(),
            Self::SqlUnaryExpression(expr) => expr.needs_parentheses(),
            Self::SqlIsExpression(expr) => expr.needs_parentheses(),
            Self::SqlIsDistinctFromExpression(expr) => expr.needs_parentheses(),
            Self::SqlPostfixNullExpression(expr) => expr.needs_parentheses(),
            Self::SqlBetweenExpression(expr) => expr.needs_parentheses(),
            Self::SqlInExpression(expr) => expr.needs_parentheses(),
            Self::SqlLikeExpression(expr) => expr.needs_parentheses(),
            // Every other expression kind is self-delimiting -- bounded by
            // its own brackets/keywords (calls, arrays, subqueries, `case`,
            // casts, `exists`/`any`/`all`, an already-parenthesized
            // expression) or is a bare atom (name, star, parameter, column
            // reference) -- so it never needs parentheses no matter what
            // it's nested inside.
            Self::SqlName(_)
            | Self::SqlStar(_)
            | Self::SqlTableStar(_)
            | Self::SqlParameterExpression(_)
            | Self::SqlTableColReference(_)
            | Self::SqlColReference(_)
            | Self::SqlCaseExpression(_)
            | Self::SqlParenthesizedExpression(_)
            | Self::SqlSubqueryExpression(_)
            | Self::SqlCallExpression(_)
            | Self::SqlWindowFunctionExpression(_)
            | Self::PsqlArrayExpression(_)
            | Self::PsqlArraySubqueryExpression(_)
            | Self::PsqlTildeArrayExpression(_)
            | Self::PsqlArraySubscriptExpression(_)
            | Self::PsqlCastExpression(_)
            | Self::SqlCollateExpression(_)
            | Self::PsqlFieldSelectionExpression(_)
            | Self::PsqlExtractExpression(_)
            | Self::PsqlPositionExpression(_)
            | Self::PsqlTrimExpression(_)
            | Self::SqlCastFunctionExpression(_)
            | Self::SqlAnyAllExpression(_)
            | Self::SqlExistsExpression(_)
            | Self::PsqlSubstringExpression(_)
            | Self::PsqlIntervalExpression(_) => false,
        }
    }
}

impl NeedsParentheses for AnySqlLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        // Unlike JS, SQL has no "directive prologue"/expression-statement
        // ambiguity for a bare string literal -- literals are always
        // atomic, regardless of what they're nested inside.
        false
    }
}

impl NeedsParentheses for SqlParenthesizedExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        // Already has its own literal parens (this impl is only ever
        // consulted for one of the rare survivors of the preprocessing
        // removal pass -- see `syntax_rewriter.rs` -- which always keeps
        // its parens for a reason, e.g. a syntax error).
        false
    }
}

impl NeedsParentheses for SqlBinaryExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        binary_like_needs_parens(self.syntax())
    }
}

impl NeedsParentheses for SqlLogicalExpression {
    fn needs_parentheses(&self) -> bool {
        // `a or b and c` already parses as `a or (b and c)` (`and` binds
        // tighter), so parentheses are never *required* here -- but a
        // logical expression directly nested inside a *different* `and`/
        // `or` is reformatted with explicit parentheses for readability,
        // the same choice `mlang_syntax`'s `MLogicalExpression` makes.
        // Nesting the *same* operator (`a and b and c`) is handled by the
        // formatter flattening it into one flat chain instead -- this impl
        // is only ever consulted for an *opaque* (different-operator or
        // non-logical) operand, never for a same-operator child.
        if let Some(parent) = self.parent::<SqlLogicalExpression>() {
            parent.operator_token().ok().map(|t| t.kind())
                != self.operator_token().ok().map(|t| t.kind())
        } else {
            binary_like_needs_parens(self.syntax())
        }
    }
}

impl NeedsParentheses for SqlUnaryExpression {
    fn needs_parentheses(&self) -> bool {
        // `- -a`/`+ +a` nested same-sign unary needs parentheses: besides
        // being confusing either way, `--` immediately introduces a line
        // comment if the two signs were ever printed adjacent to each
        // other without a parenthesis or space between them.
        if let Some(parent) = self.parent::<SqlUnaryExpression>() {
            let parent_operator = parent.operator_token().ok().map(|t| t.kind());
            let operator = self.operator_token().ok().map(|t| t.kind());
            matches!(operator, Some(T![-]) | Some(T![+])) && parent_operator == operator
        } else {
            false
        }
    }
}

impl NeedsParentheses for SqlIsExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        precedence_needs_parens(self.syntax(), OperatorPrecedence::IsNull)
    }
}

impl NeedsParentheses for SqlIsDistinctFromExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        precedence_needs_parens(self.syntax(), OperatorPrecedence::IsNull)
    }
}

impl NeedsParentheses for SqlPostfixNullExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        precedence_needs_parens(self.syntax(), OperatorPrecedence::IsNull)
    }
}

impl NeedsParentheses for SqlBetweenExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        precedence_needs_parens(self.syntax(), OperatorPrecedence::Predicate)
    }
}

impl NeedsParentheses for SqlInExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        precedence_needs_parens(self.syntax(), OperatorPrecedence::Predicate)
    }
}

impl NeedsParentheses for SqlLikeExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        precedence_needs_parens(self.syntax(), OperatorPrecedence::Predicate)
    }
}

/// The fixed precedence tier of an already-known-to-be-a-binary-or-logical
/// operator token, or `None` if `kind` isn't one (e.g. it's some other
/// token entirely, or the tree is malformed and the field is missing).
fn operator_precedence(kind: Option<SqlSyntaxKind>) -> Option<OperatorPrecedence> {
    let kind = kind?;
    if matches!(kind, T![and] | T![or]) {
        return Some(if kind == T![and] {
            OperatorPrecedence::LogicalAnd
        } else {
            OperatorPrecedence::LogicalOr
        });
    }
    OperatorPrecedence::try_from_binary_operator(kind)
}

/// The precedence tier `expr` prints at, used to decide whether a
/// precedence-bearing expression needs parentheses given its parent. Every
/// expression kind not explicitly listed here is self-delimiting (own
/// brackets/keywords), so it's always treated as binding at least as
/// tightly as anything else -- [OperatorPrecedence::Primary].
fn expression_precedence(expr: &AnySqlExpression) -> OperatorPrecedence {
    match expr {
        AnySqlExpression::SqlBinaryExpression(binary) => {
            operator_precedence(binary.operator_token().ok().map(|t| t.kind()))
                .unwrap_or(OperatorPrecedence::lowest())
        }
        AnySqlExpression::SqlLogicalExpression(logical) => {
            operator_precedence(logical.operator_token().ok().map(|t| t.kind()))
                .unwrap_or(OperatorPrecedence::lowest())
        }
        AnySqlExpression::SqlUnaryExpression(unary) => {
            if unary.operator_token().ok().map(|t| t.kind()) == Some(T![not]) {
                OperatorPrecedence::Not
            } else {
                OperatorPrecedence::Unary
            }
        }
        AnySqlExpression::SqlIsExpression(_)
        | AnySqlExpression::SqlIsDistinctFromExpression(_)
        | AnySqlExpression::SqlPostfixNullExpression(_) => OperatorPrecedence::IsNull,
        AnySqlExpression::SqlBetweenExpression(_)
        | AnySqlExpression::SqlInExpression(_)
        | AnySqlExpression::SqlLikeExpression(_) => OperatorPrecedence::Predicate,
        _ => OperatorPrecedence::Primary,
    }
}

/// Returns `true` if a precedence-bearing node (fixed at `precedence`, e.g.
/// [OperatorPrecedence::IsNull] for [SqlIsExpression]) needs
/// parentheses given its parent: whenever the parent binds *tighter* than
/// `precedence`, wrapping is required to preserve the original grouping.
fn precedence_needs_parens(node: &SqlSyntaxNode, precedence: OperatorPrecedence) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    // Already wrapped in literal parentheses that are kept as written.
    if parent.kind() == SqlSyntaxKind::SQL_PARENTHESIZED_EXPRESSION {
        return false;
    }
    if let Some(needs_parens) = keyword_delimited_needs_parens(node, &parent, precedence) {
        return needs_parens;
    }
    let Some(parent_expr) = AnySqlExpression::cast(parent) else {
        return false;
    };
    expression_precedence(&parent_expr) > precedence
}

/// Operands of `extract(... from ...)`, `position(... in ...)`, `trim(...)`,
/// `substring(...)` and `cast(... as ...)` are delimited by the call's own
/// keywords and parentheses, so they never need extra parentheses -- except
/// `position`'s first operand, which is parsed above `IN`'s precedence.
fn keyword_delimited_needs_parens(
    node: &SqlSyntaxNode,
    parent: &SqlSyntaxNode,
    precedence: OperatorPrecedence,
) -> Option<bool> {
    match parent.kind() {
        SqlSyntaxKind::PSQL_POSITION_EXPRESSION => {
            let is_first_operand = parent
                .children()
                .find(|child| AnySqlExpression::can_cast(child.kind()))
                .is_some_and(|first| &first == node);
            Some(is_first_operand && precedence <= OperatorPrecedence::Predicate)
        }
        SqlSyntaxKind::PSQL_EXTRACT_EXPRESSION
        | SqlSyntaxKind::PSQL_TRIM_EXPRESSION
        | SqlSyntaxKind::PSQL_SUBSTRING_EXPRESSION
        | SqlSyntaxKind::PSQL_SUBSTRING_FROM_CLAUSE
        | SqlSyntaxKind::PSQL_SUBSTRING_FOR_CLAUSE
        | SqlSyntaxKind::SQL_CAST_FUNCTION_EXPRESSION => Some(false),
        _ => None,
    }
}

/// Implements the rules shared by [SqlBinaryExpression] and
/// [SqlLogicalExpression] (when the latter isn't already handled by its
/// own and/or-mismatch rule).
fn binary_like_needs_parens(node: &SqlSyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    // Already wrapped in literal parentheses that are kept as written.
    if parent.kind() == SqlSyntaxKind::SQL_PARENTHESIZED_EXPRESSION {
        return false;
    }
    let Some(child) = AnySqlExpression::cast_ref(node) else {
        return false;
    };
    let precedence = expression_precedence(&child);
    if let Some(needs_parens) = keyword_delimited_needs_parens(node, &parent, precedence) {
        return needs_parens;
    }
    let Some(parent_expr) = AnySqlExpression::cast(parent) else {
        return false;
    };

    let parent_precedence = expression_precedence(&parent_expr);

    // Parent binds tighter -- parentheses are necessary to not change the
    // semantics when re-parsing.
    if parent_precedence > precedence {
        return true;
    }

    if parent_precedence == precedence {
        // Our parser only ever produces a *left*-recursive tree for a run
        // of same-precedence operators (`a - b - c` is always `(a - b) -
        // c`), so a same-precedence node only ends up on the parent's
        // *right* side when the original source had explicit parentheses
        // forcing that grouping (`a - (b - c)`) -- and since that changes
        // the result for any non-associative operator (and this rule
        // doesn't attempt to prove associativity/commutativity per
        // operator pair), the parentheses must be kept.
        let is_right = match &parent_expr {
            AnySqlExpression::SqlBinaryExpression(parent_binary) => parent_binary
                .right()
                .is_ok_and(|right| right.syntax() == node),
            AnySqlExpression::SqlLogicalExpression(parent_logical) => parent_logical
                .right()
                .is_ok_and(|right| right.syntax() == node),
            _ => false,
        };

        return is_right;
    }

    false
}
