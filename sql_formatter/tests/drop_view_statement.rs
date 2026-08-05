#[macro_use]
mod helper;

#[test]
fn format_drop_view_bare_name() {
    assert_fmt!(
        r#"--
drop view foo
"#
    );
}

#[test]
fn format_drop_view_if_exists() {
    assert_fmt!(
        r#"--
drop view if exists foo;
"#
    );
}

#[test]
fn format_drop_view_multiple_names() {
    assert_fmt!(
        r#"--
drop view foo, bar
"#
    );
}

#[test]
fn format_drop_view_cascade() {
    assert_fmt!(
        r#"--
drop view if exists foo cascade;
"#
    );
}

#[test]
fn format_drop_view_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
DROP   VIEW   IF EXISTS   foo,bar   CASCADE;
"#,
        r#"--
drop view if exists foo, bar cascade;
"#
    );
}
