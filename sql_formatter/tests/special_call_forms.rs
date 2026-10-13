#[macro_use]
mod helper;

#[test]
fn format_special_call_forms() {
    assert_fmt!(
        r#"--
select extract(epoch from now() - t.c), position('a' in s), trim(both ' ' from s) from t
select position((a = b) in s), substring(a || b from 1 for 2), cast(a + b as int) from t
"#
    );
}

#[test]
fn format_special_call_forms_normalize_keyword_case_and_spacing() {
    assert_fmt_eq!(
        r#"--
SELECT extract( year FROM d ), position( 'a' IN s ), trim( LEADING '0' FROM s ) FROM t
"#,
        r#"--
select extract(year from d), position('a' in s), trim(leading '0' from s) from t
"#
    );
}

#[test]
fn format_long_select_with_special_call_forms_wraps() {
    assert_fmt_eq!(
        r#"--
select extract(year from age(coalesce(employees.dismissed, now()::date), employees.hired)) as experience_years, position(lower(employees.code) in lower(employees.full_name)) as code_position from employees
"#,
        r#"--
select
	extract(year from age(coalesce(employees.dismissed, now()::date), employees.hired)) as experience_years,
	position(lower(employees.code) in lower(employees.full_name)) as code_position
from employees
"#
    );
}
