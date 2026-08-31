#[macro_use]
mod helper;

#[test]
fn format_drop_table_bare_name() {
    assert_fmt!(
        r#"--
drop table foo
"#
    );
}

#[test]
fn format_drop_table_if_exists() {
    assert_fmt!(
        r#"--
drop table if exists foo;
"#
    );
}

#[test]
fn format_drop_table_qualified_name() {
    assert_fmt!(
        r#"--
drop table myschema.foo
"#
    );
}

#[test]
fn format_drop_table_multiple_names() {
    assert_fmt!(
        r#"--
drop table foo, bar, baz
"#
    );
}

#[test]
fn format_drop_table_cascade() {
    assert_fmt!(
        r#"--
drop table if exists foo cascade;
"#
    );
}

#[test]
fn format_drop_table_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
DROP   TABLE   IF EXISTS   foo,bar   CASCADE;
"#,
        r#"--
drop table if exists foo, bar cascade;
"#
    );
}

#[test]
fn format_drop_table_names_wrap_when_too_long() {
    assert_fmt!(
        r#"--
drop table
	really_long_table_name_number_one, really_long_table_name_number_two,
	really_long_table_name_number_three, really_long_table_name_number_four
"#
    );
}
