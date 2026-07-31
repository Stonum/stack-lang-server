#[macro_use]
mod helper;

#[test]
fn format_is_null() {
    assert_fmt!(
        r#"--
select a from t where a is null
"#
    );
}

#[test]
fn format_is_not_null() {
    assert_fmt!(
        r#"--
select a from t where a is not null
"#
    );
}

#[test]
fn format_is_null_normalizes_case() {
    assert_fmt_eq!(
        r#"--
select a from t where a IS NOT NULL
"#,
        r#"--
select a from t where a is not null
"#
    );
}

#[test]
fn format_between() {
    assert_fmt!(
        r#"--
select a from t where a between 1 and 10
"#
    );
}

#[test]
fn format_not_between() {
    assert_fmt!(
        r#"--
select a from t where a not between 1 and 10
"#
    );
}

#[test]
fn format_in_value_list() {
    assert_fmt!(
        r#"--
select a from t where a in (1, 2, 3)
"#
    );
}

#[test]
fn format_not_in_value_list() {
    assert_fmt!(
        r#"--
select a from t where a not in (1, 2, 3)
"#
    );
}

#[test]
fn format_in_subquery() {
    // `PsqlSubqueryExpression` always block-indents its query, so this
    // hard break propagates outward and expands the enclosing clauses.
    assert_fmt!(
        r#"--
select a
from t
where a in (
	select b from u
)
"#
    );
}

#[test]
fn format_like() {
    assert_fmt!(
        r#"--
select a from t where a like '%x%'
"#
    );
}

#[test]
fn format_not_like() {
    assert_fmt!(
        r#"--
select a from t where a not like '%x%'
"#
    );
}

#[test]
fn format_ilike() {
    assert_fmt!(
        r#"--
select a from t where a ilike '%x%'
"#
    );
}

#[test]
fn format_like_normalizes_case() {
    assert_fmt_eq!(
        r#"--
select a from t where a NOT ILIKE '%x%'
"#,
        r#"--
select a from t where a not ilike '%x%'
"#
    );
}
