#[macro_use]
mod helper;

#[test]
fn format_any_with_subquery() {
    assert_fmt!(
        r#"--
select a from t where a = any(select b from u)
"#
    );
}

#[test]
fn format_all_with_subquery() {
    assert_fmt!(
        r#"--
select a from t where a <> all(select b from u)
"#
    );
}

#[test]
fn format_some_with_subquery() {
    assert_fmt!(
        r#"--
select a from t where a = some(select b from u)
"#
    );
}

#[test]
fn format_any_with_array_expression() {
    assert_fmt!(
        r#"--
select a from t where a = any(array[1, 2, 3])
"#
    );
}

#[test]
fn format_any_normalizes_case() {
    // Formatted directly (not through a `PsqlBinaryExpression` ancestor,
    // which still falls back to verbatim -- see the project roadmap) to
    // isolate this node's own casing normalization.
    assert_fmt_node!(
        "select a from t where a = ANY(select b from u)",
        psql_syntax::PsqlSyntaxKind::PSQL_ANY_ALL_EXPRESSION,
        "any(\n\tselect b from u\n)"
    );
}
