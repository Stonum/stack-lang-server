#[macro_use]
mod helper;

#[test]
fn format_is_predicates() {
    assert_fmt!(
        r#"--
select a from t where a is true and b is not false
select a from t where c is unknown
select a from t where a isnull or b notnull
select a from t where a is distinct from b or c is not distinct from d
"#
    );
}

#[test]
fn format_is_predicates_normalize_case_and_keep_needed_parens() {
    assert_fmt_eq!(
        r#"--
SELECT (a IS TRUE) = b, a = (b IS TRUE), a IS DISTINCT FROM (b AND c), NOT a ISNULL FROM t
"#,
        r#"--
select (a is true) = b, a = (b is true), a is distinct from (b and c), not a isnull from t
"#
    );
}

#[test]
fn format_long_condition_with_is_predicates_wraps() {
    assert_fmt_eq!(
        r#"--
select a from t where first_long_column_name is distinct from second_long_column_name and third_long_column_name is not distinct from fourth_long_column_name and fifth_long_column isnull
"#,
        r#"--
select a
from t
where first_long_column_name is distinct from second_long_column_name
	and third_long_column_name is not distinct from fourth_long_column_name
	and fifth_long_column isnull
"#
    );
}
