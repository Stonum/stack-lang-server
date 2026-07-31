use biome_rowan::AstNode;

use crate::{
    AnyPsqlExpression, AnyPsqlLiteralExpression, OperatorPrecedence, PsqlBetweenExpression,
    PsqlBinaryExpression, PsqlInExpression, PsqlIsNullExpression, PsqlLikeExpression,
    PsqlLogicalExpression, PsqlSyntaxKind, PsqlSyntaxNode, PsqlUnaryExpression,
};

use super::NeedsParentheses;

impl NeedsParentheses for AnyPsqlExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::AnyPsqlLiteralExpression(expr) => expr.needs_parentheses(),
            Self::PsqlBinaryExpression(expr) => expr.needs_parentheses(),
            Self::PsqlLogicalExpression(expr) => expr.needs_parentheses(),
            Self::PsqlUnaryExpression(expr) => expr.needs_parentheses(),
            Self::PsqlIsNullExpression(expr) => expr.needs_parentheses(),
            Self::PsqlBetweenExpression(expr) => expr.needs_parentheses(),
            Self::PsqlInExpression(expr) => expr.needs_parentheses(),
            Self::PsqlLikeExpression(expr) => expr.needs_parentheses(),
            // Every other expression kind is self-delimiting -- bounded by
            // its own brackets/keywords (calls, arrays, subqueries, `case`,
            // casts, `exists`/`any`/`all`, an already-parenthesized
            // expression) or is a bare atom (name, star, parameter, column
            // reference) -- so it never needs parentheses no matter what
            // it's nested inside.
            Self::PsqlName(_)
            | Self::PsqlStar(_)
            | Self::PsqlParameterExpression(_)
            | Self::PsqlTableColReference(_)
            | Self::PsqlColReference(_)
            | Self::PsqlCaseExpression(_)
            | Self::PsqlParenthesizedExpression(_)
            | Self::PsqlSubqueryExpression(_)
            | Self::PsqlCallExpression(_)
            | Self::PsqlWindowFunctionExpression(_)
            | Self::PsqlArrayExpression(_)
            | Self::PsqlTildeArrayExpression(_)
            | Self::PsqlArraySubscriptExpression(_)
            | Self::PsqlCastExpression(_)
            | Self::PsqlCastFunctionExpression(_)
            | Self::PsqlAnyAllExpression(_)
            | Self::PsqlExistsExpression(_) => false,
        }
    }
}

impl NeedsParentheses for AnyPsqlLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        // Unlike JS, SQL has no "directive prologue"/expression-statement
        // ambiguity for a bare string literal -- literals are always
        // atomic, regardless of what they're nested inside.
        false
    }
}

impl NeedsParentheses for PsqlBinaryExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        binary_like_needs_parens(self.syntax())
    }
}

impl NeedsParentheses for PsqlLogicalExpression {
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
        if let Some(parent) = self.parent::<PsqlLogicalExpression>() {
            parent.operator_token().ok().map(|t| t.kind())
                != self.operator_token().ok().map(|t| t.kind())
        } else {
            binary_like_needs_parens(self.syntax())
        }
    }
}

impl NeedsParentheses for PsqlUnaryExpression {
    fn needs_parentheses(&self) -> bool {
        // `- -a`/`+ +a` nested same-sign unary needs parentheses: besides
        // being confusing either way, `--` immediately introduces a line
        // comment if the two signs were ever printed adjacent to each
        // other without a parenthesis or space between them.
        if let Some(parent) = self.parent::<PsqlUnaryExpression>() {
            let parent_operator = parent.operator_token().ok().map(|t| t.kind());
            let operator = self.operator_token().ok().map(|t| t.kind());
            matches!(operator, Some(T![-]) | Some(T![+])) && parent_operator == operator
        } else {
            false
        }
    }
}

impl NeedsParentheses for PsqlIsNullExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        precedence_needs_parens(self.syntax(), OperatorPrecedence::IsNull)
    }
}

impl NeedsParentheses for PsqlBetweenExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        precedence_needs_parens(self.syntax(), OperatorPrecedence::Predicate)
    }
}

impl NeedsParentheses for PsqlInExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        precedence_needs_parens(self.syntax(), OperatorPrecedence::Predicate)
    }
}

impl NeedsParentheses for PsqlLikeExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        precedence_needs_parens(self.syntax(), OperatorPrecedence::Predicate)
    }
}

/// The fixed precedence tier of an already-known-to-be-a-binary-or-logical
/// operator token, or `None` if `kind` isn't one (e.g. it's some other
/// token entirely, or the tree is malformed and the field is missing).
fn operator_precedence(kind: Option<PsqlSyntaxKind>) -> Option<OperatorPrecedence> {
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
fn expression_precedence(expr: &AnyPsqlExpression) -> OperatorPrecedence {
    match expr {
        AnyPsqlExpression::PsqlBinaryExpression(binary) => {
            operator_precedence(binary.operator_token().ok().map(|t| t.kind()))
                .unwrap_or(OperatorPrecedence::lowest())
        }
        AnyPsqlExpression::PsqlLogicalExpression(logical) => {
            operator_precedence(logical.operator_token().ok().map(|t| t.kind()))
                .unwrap_or(OperatorPrecedence::lowest())
        }
        AnyPsqlExpression::PsqlUnaryExpression(unary) => {
            if unary.operator_token().ok().map(|t| t.kind()) == Some(T![not]) {
                OperatorPrecedence::Not
            } else {
                OperatorPrecedence::Unary
            }
        }
        AnyPsqlExpression::PsqlIsNullExpression(_) => OperatorPrecedence::IsNull,
        AnyPsqlExpression::PsqlBetweenExpression(_)
        | AnyPsqlExpression::PsqlInExpression(_)
        | AnyPsqlExpression::PsqlLikeExpression(_) => OperatorPrecedence::Predicate,
        _ => OperatorPrecedence::Primary,
    }
}

/// Returns `true` if a precedence-bearing node (fixed at `precedence`, e.g.
/// [OperatorPrecedence::IsNull] for [PsqlIsNullExpression]) needs
/// parentheses given its parent: whenever the parent binds *tighter* than
/// `precedence`, wrapping is required to preserve the original grouping.
fn precedence_needs_parens(node: &PsqlSyntaxNode, precedence: OperatorPrecedence) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    let Some(parent_expr) = AnyPsqlExpression::cast(parent) else {
        return false;
    };
    expression_precedence(&parent_expr) > precedence
}

/// Implements the rules shared by [PsqlBinaryExpression] and
/// [PsqlLogicalExpression] (when the latter isn't already handled by its
/// own and/or-mismatch rule).
fn binary_like_needs_parens(node: &PsqlSyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    let Some(child) = AnyPsqlExpression::cast_ref(node) else {
        return false;
    };
    let Some(parent_expr) = AnyPsqlExpression::cast(parent) else {
        return false;
    };

    let precedence = expression_precedence(&child);
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
            AnyPsqlExpression::PsqlBinaryExpression(parent_binary) => parent_binary
                .right()
                .is_ok_and(|right| right.syntax() == node),
            AnyPsqlExpression::PsqlLogicalExpression(parent_logical) => parent_logical
                .right()
                .is_ok_and(|right| right.syntax() == node),
            _ => false,
        };

        return is_right;
    }

    false
}
