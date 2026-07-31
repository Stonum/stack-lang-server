use psql_parser::parse;
use psql_syntax::{
    AstNode, NeedsParentheses, PsqlBinaryExpression, PsqlFileSource, PsqlInExpression,
    PsqlIsNullExpression, PsqlLanguage, PsqlLogicalExpression, PsqlUnaryExpression,
};

/// Finds the (single, expected-unique) node of kind `T` whose trimmed
/// source text equals `text`, so each test can pinpoint the exact node
/// it's asserting on rather than relying on traversal order.
fn find_by_text<T: AstNode<Language = PsqlLanguage>>(src: &str, text: &str) -> T {
    let tree = parse(src, PsqlFileSource::script());
    assert!(
        !tree.has_errors(),
        "parse errors for {src:?}: {:?}",
        tree.diagnostics()
    );
    tree.syntax()
        .descendants()
        .filter_map(T::cast)
        .find(|node| node.syntax().text_trimmed() == text)
        .unwrap_or_else(|| panic!("no node with text {text:?} found in {src:?}"))
}

#[test]
fn binary_expression_bound_tighter_than_parent_does_not_need_parens() {
    // `a + b * c` -- the `*` naturally nests as the (right) child of `+`
    // since it binds tighter; no parentheses needed to preserve the
    // original grouping.
    let inner = find_by_text::<PsqlBinaryExpression>("select a + b * c", "b * c");
    assert!(!inner.needs_parentheses());
}

#[test]
fn binary_expression_at_the_root_does_not_need_parens() {
    // No expression parent at all (parent is `PsqlWhereClause`).
    let root = find_by_text::<PsqlBinaryExpression>("select a from t where a > 0", "a > 0");
    assert!(!root.needs_parentheses());
}

#[test]
fn logical_expression_mismatched_operator_needs_parens_or_parent() {
    // `a or b and c` already parses as `a or (b and c)` (`and` binds
    // tighter) -- parentheses aren't semantically required, but are added
    // for readability when mixing `and`/`or` without an explicit grouping.
    let inner = find_by_text::<PsqlLogicalExpression>("select a or b and c", "b and c");
    assert!(inner.needs_parentheses());
}

#[test]
fn logical_expression_mismatched_operator_needs_parens_and_parent() {
    // `a and b or c` parses as `(a and b) or c` -- same readability rule,
    // this time with the mismatched child on the left.
    let inner = find_by_text::<PsqlLogicalExpression>("select a and b or c", "a and b");
    assert!(inner.needs_parentheses());
}

#[test]
fn logical_expression_same_operator_does_not_need_parens() {
    // `a or b or c` -- left-recursive same-operator chain; the formatter
    // flattens this away before ever asking, but the trait itself must
    // also agree no parentheses are needed (`or` is associative).
    let inner = find_by_text::<PsqlLogicalExpression>("select a or b or c", "a or b");
    assert!(!inner.needs_parentheses());
}

#[test]
fn unary_same_sign_nested_needs_parens() {
    // `- -a` -- besides being confusing, printing the two signs adjacent
    // to each other without a separator would produce `--`, a line
    // comment starter.
    let inner = find_by_text::<PsqlUnaryExpression>("select - -a", "-a");
    assert!(inner.needs_parentheses());
}

#[test]
fn unary_mixed_sign_nested_does_not_need_parens() {
    // `- +a` -- no `--`/`++` visual-ambiguity risk when the signs differ.
    let inner = find_by_text::<PsqlUnaryExpression>("select - +a", "+a");
    assert!(!inner.needs_parentheses());
}

#[test]
fn unary_not_nested_does_not_need_parens() {
    // `not not a` -- no comment-ambiguity risk for keyword `not`.
    let inner = find_by_text::<PsqlUnaryExpression>("select not not a", "not a");
    assert!(!inner.needs_parentheses());
}

#[test]
fn is_null_expression_as_child_of_tighter_binary_needs_parens() {
    // `a is null + b` -- `is null` binds looser than `+`, so the
    // `IsNullExpression` ends up as the (left) child of the `+`
    // `BinaryExpression`; parentheses are required to preserve the
    // original grouping.
    let inner = find_by_text::<PsqlIsNullExpression>("select a is null + b", "a is null");
    assert!(inner.needs_parentheses());
}

#[test]
fn is_null_expression_as_child_of_looser_logical_does_not_need_parens() {
    // `a is null and b` -- `is null` already binds tighter than `and`, so
    // no parentheses are needed to keep `a is null` grouped together.
    let inner = find_by_text::<PsqlIsNullExpression>("select a is null and b", "a is null");
    assert!(!inner.needs_parentheses());
}

#[test]
fn in_expression_as_child_of_tighter_binary_needs_parens() {
    // `a in (1, 2) + 1` -- `in` binds looser than `+`, so the
    // `InExpression` ends up as the child of the `+` `BinaryExpression`.
    let inner = find_by_text::<PsqlInExpression>("select a in (1, 2) + 1", "a in (1, 2)");
    assert!(inner.needs_parentheses());
}
