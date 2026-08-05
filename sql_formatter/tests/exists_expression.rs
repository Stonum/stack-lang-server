#[macro_use]
mod helper;

#[test]
fn format_exists_with_subquery() {
    assert_fmt!(
        r#"--
select a from t where exists (select 1 from u where u.a = t.a)
"#
    );
}

#[test]
fn format_exists_normalizes_case() {
    assert_fmt_node!(
        "select a from t where EXISTS(select b from u)",
        sql_syntax::SqlSyntaxKind::SQL_EXISTS_EXPRESSION,
        "exists (select b from u)"
    );
}
