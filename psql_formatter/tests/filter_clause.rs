#[macro_use]
mod helper;

#[test]
fn format_filter_clause_on_aggregate() {
    assert_fmt!(
        r#"--
select count(x) filter (where a > 1) from t
"#
    );
}

#[test]
fn format_filter_clause_multiple_aggregates() {
    assert_fmt!(
        r#"--
select count(x) filter (where a > 1), sum(y) filter (where b < 2) from t
"#
    );
}

#[test]
fn format_filter_clause_with_window_function() {
    assert_fmt!(
        r#"--
select count(x) filter (where a > 1) over (partition by b) from t
"#
    );
}

#[test]
fn format_filter_clause_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
select count(x) FILTER(WHERE a > 1) from t;
"#,
        r#"--
select count(x) filter (where a > 1) from t;
"#
    );
}
