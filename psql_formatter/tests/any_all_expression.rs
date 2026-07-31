#[macro_use]
mod helper;

#[test]
fn format_any_with_subquery() {
    // `PsqlSubqueryExpression` always block-indents its query (see its own
    // formatter), regardless of length -- so `any(select ...)` always
    // spans multiple lines, and that hard break propagates outward,
    // expanding the enclosing `select`/`from`/`where` clauses onto their
    // own lines too.
    assert_fmt!(
        r#"--
select a
from t
where a = any(
	select b from u
)
"#
    );
}

#[test]
fn format_all_with_subquery() {
    assert_fmt!(
        r#"--
select a
from t
where a <> all(
	select b from u
)
"#
    );
}

#[test]
fn format_some_with_subquery() {
    assert_fmt!(
        r#"--
select a
from t
where a = some(
	select b from u
)
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
    // Formatted directly (isolating just this node, bypassing the wrapping
    // `select`/`from`/`where` clauses that a `PsqlSubqueryExpression`'s
    // unconditional block-indent would otherwise force to expand too) to
    // check this node's own casing normalization in isolation.
    assert_fmt_node!(
        "select a from t where a = ANY(select b from u)",
        psql_syntax::PsqlSyntaxKind::PSQL_ANY_ALL_EXPRESSION,
        "any(\n\tselect b from u\n)"
    );
}
