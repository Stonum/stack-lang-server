#[macro_use]
mod helper;

#[test]
fn format_grant_all_on_table_to_public() {
    assert_fmt!(
        r#"--
grant all on table foo to public
"#
    );
}

#[test]
fn format_grant_without_table_keyword() {
    assert_fmt!(
        r#"--
grant all on foo to public
"#
    );
}

#[test]
fn format_grant_multiple_objects_and_grantees() {
    assert_fmt!(
        r#"--
grant all on table foo, bar to public, other_role;
"#
    );
}

#[test]
fn format_grant_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
GRANT   ALL   ON   TABLE   foo   TO   public;
"#,
        r#"--
grant all on table foo to public;
"#
    );
}
