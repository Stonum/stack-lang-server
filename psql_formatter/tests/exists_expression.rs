#[macro_use]
mod helper;

#[test]
fn format_exists_with_subquery() {
    assert_fmt!(
        r#"--
select a
from t
where exists (
	select 1 from u where u.a = t.a
)
"#
    );
}

#[test]
fn format_exists_normalizes_case() {
    // Formatted directly (not through a `PsqlLogicalExpression` ancestor,
    // which still falls back to verbatim -- see the project roadmap) to
    // isolate this node's own casing normalization.
    assert_fmt_node!(
        "select a from t where EXISTS(select b from u)",
        psql_syntax::PsqlSyntaxKind::PSQL_EXISTS_EXPRESSION,
        "exists (\n\tselect b from u\n)"
    );
}
