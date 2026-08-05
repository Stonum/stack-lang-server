#[macro_use]
mod helper;

#[test]
fn format_substring_from_position() {
    assert_fmt!(
        r#"--
select substring(str from 3)
"#
    );
}

#[test]
fn format_substring_from_for() {
    assert_fmt!(
        r#"--
select substring(str from 3 for 5)
"#
    );
}

#[test]
fn format_substring_normalizes_keyword_case_but_not_the_function_name() {
    // `from`/`for` are real keywords and get lowercased; `substring`
    // itself isn't a reserved keyword in Postgres, so its casing is
    // preserved verbatim -- same convention as `old`/`new` in a trigger's
    // REFERENCING clause.
    assert_fmt_eq!(
        r#"--
select SUBSTRING(str FROM 3 FOR 5)
"#,
        r#"--
select SUBSTRING(str from 3 for 5)
"#
    );
}

#[test]
fn format_substring_comma_form_is_unaffected() {
    assert_fmt!(
        r#"--
select substring(str, 1, 5)
"#
    );
}
