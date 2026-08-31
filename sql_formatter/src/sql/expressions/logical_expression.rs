use crate::comments::SqlComments;
use crate::prelude::*;
use biome_formatter::write;
use biome_rowan::SyntaxResult;
use sql_syntax::{
    AnySqlExpression, NeedsParentheses, SqlLogicalExpression, SqlLogicalExpressionFields,
    SqlSyntaxKind, SqlSyntaxToken,
};

/// Unwraps any real (source) parens around `expr`, repeatedly -- `((x))`
/// unwraps all the way down to `x`. Used to see through grouping parens
/// when deciding operand complexity, since `(a and b)` and `a and b` are
/// the same condition as far as that's concerned.
fn unwrap_parens(mut expr: AnySqlExpression) -> AnySqlExpression {
    while let AnySqlExpression::SqlParenthesizedExpression(parenthesized) = &expr {
        match parenthesized.expression() {
            Ok(inner) => expr = inner,
            Err(_) => break,
        }
    }
    expr
}

/// Style rule 5's "a chain of at most two conditions never wraps" exemption
/// only holds when every condition is a single, self-contained predicate.
/// An operand that's itself a nested `and`/`or` group one level down --
/// `(a between b and c) or (d between e and f)`, say -- can still explode
/// past any reasonable line width even though *this* level only sees two
/// operands, since each one can hide arbitrary further nesting. A single
/// level of nesting with only plain leaves inside (`(a and b) or c`) is
/// still exempt -- only nesting *within* the nested group disqualifies it.
fn is_complex_condition(expr: &AnySqlExpression) -> bool {
    let AnySqlExpression::SqlLogicalExpression(logical) = unwrap_parens(expr.clone()) else {
        return false;
    };
    let is_nested_logical = |side: SyntaxResult<AnySqlExpression>| {
        side.is_ok_and(|side| {
            matches!(
                unwrap_parens(side),
                AnySqlExpression::SqlLogicalExpression(_)
            )
        })
    };
    is_nested_logical(logical.left()) || is_nested_logical(logical.right())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlLogicalExpression;
impl FormatNodeRule<SqlLogicalExpression> for FormatSqlLogicalExpression {
    fn fmt_fields(&self, node: &SqlLogicalExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let operator_kind = node.operator_token()?.kind();

        // Flatten a left-associative run of the *same* and/or operator into
        // a flat operand list -- `a and b and c` is parsed as
        // `(a and b) and c`, so this walks down `left` while it's still the
        // same operator, treating anything else (a different operator, or a
        // non-logical expression) as one opaque operand.
        let mut operands = Vec::new();
        collect_operands(node.left(), operator_kind, &mut operands, f.comments())?;
        operands.push((Some(node.operator_token()?), node.right()?));

        let (_, first) = &operands[0];
        write!(f, [first.format()])?;

        if operands.len() <= 2 && operands.iter().all(|(_, expr)| !is_complex_condition(expr)) {
            // Style rule 5: a chain of at most two conditions never wraps,
            // no matter how long the line ends up being -- unless one of the
            // two is itself a nested and/or group (see [is_complex_condition]),
            // which can hide arbitrary further nesting behind what looks
            // like a single operand here.
            for (operator, expr) in &operands[1..] {
                let operator = operator
                    .as_ref()
                    .expect("non-first operand has an operator");
                write!(f, [space(), operator.format(), space(), expr.format()])?;
            }
            Ok(())
        } else {
            // More than two conditions: every condition after the first
            // always gets its own line, indented one level deeper than the
            // line being continued, with the operator leading (not
            // trailing, unlike list commas).
            write!(
                f,
                [indent(&format_once(|f| {
                    for (operator, expr) in &operands[1..] {
                        let operator = operator
                            .as_ref()
                            .expect("non-first operand has an operator");
                        write!(
                            f,
                            [hard_line_break(), operator.format(), space(), expr.format()]
                        )?;
                    }
                    Ok(())
                }))]
            )
        }
    }

    fn needs_parentheses(&self, item: &SqlLogicalExpression) -> bool {
        item.needs_parentheses()
    }
}

fn collect_operands(
    expr: SyntaxResult<AnySqlExpression>,
    operator_kind: SqlSyntaxKind,
    out: &mut Vec<(Option<SqlSyntaxToken>, AnySqlExpression)>,
    comments: &SqlComments,
) -> FormatResult<()> {
    match expr? {
        AnySqlExpression::SqlLogicalExpression(logical)
            if logical.operator_token()?.kind() == operator_kind
                && !comments.has_comments(logical.syntax()) =>
        {
            // This intermediate node is flattened away -- it never gets its
            // own `.format()` call -- but the formatter still requires
            // every node to be checked for suppression comments.
            comments.mark_suppression_checked(logical.syntax());

            let SqlLogicalExpressionFields {
                left,
                operator_token,
                right,
            } = logical.as_fields();
            collect_operands(left, operator_kind, out, comments)?;
            out.push((Some(operator_token?), right?));
        }
        other => out.push((None, other)),
    }
    Ok(())
}
