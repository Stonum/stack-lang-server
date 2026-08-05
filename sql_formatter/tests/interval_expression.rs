#[macro_use]
mod helper;

#[test]
fn format_interval_literal_addition() {
    assert_fmt!(
        r#"--
select now() + interval '1 day' from t
"#
    );
}

#[test]
fn format_interval_literal_subtraction() {
    assert_fmt!(
        r#"--
select a from t where dt < now() - interval '1 second'
"#
    );
}

#[test]
fn format_interval_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
select INTERVAL    '1 day' from t;
"#,
        r#"--
select interval '1 day' from t;
"#
    );
}

#[test]
fn format_interval_as_type_name_still_works() {
    assert_fmt!(
        r#"--
select a::interval from t
"#
    );
}
