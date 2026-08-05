#[macro_use]
mod helper;

#[test]
fn format_create_view_simple() {
    assert_fmt!(
        r#"--
create view foo as select a from t
"#
    );
}

#[test]
fn format_create_or_replace_view() {
    assert_fmt!(
        r#"--
create or replace view foo as select a from t
"#
    );
}

#[test]
fn format_create_view_qualified_name() {
    assert_fmt!(
        r#"--
create view myschema.foo as select a from t
"#
    );
}

#[test]
fn format_create_view_with_options() {
    assert_fmt!(
        r#"--
create view foo with (security_invoker = true) as select a from t;
"#
    );
}

#[test]
fn format_create_view_with_multiple_options() {
    assert_fmt!(
        r#"--
create view foo with (security_invoker = true, check_option = cascaded) as select a from t
"#
    );
}

#[test]
fn format_create_view_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE   OR REPLACE   VIEW foo   WITH(security_invoker=true)   AS select a from t;
"#,
        r#"--
create or replace view foo with (security_invoker = true) as select a from t;
"#
    );
}
