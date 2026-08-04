#[macro_use]
mod helper;

#[test]
fn format_table_star_as_call_argument() {
    assert_fmt!(
        r#"--
select count(t.*) from t
"#
    );
}

#[test]
fn format_table_star_alongside_other_call_arguments() {
    assert_fmt!(
        r#"--
select f(t.*, 1) from t
"#
    );
}

#[test]
fn format_schema_qualified_table_star_as_call_argument() {
    assert_fmt!(
        r#"--
select count(s.t.*) from s.t
"#
    );
}

#[test]
fn format_table_star_still_works_in_select_item_position() {
    assert_fmt!(
        r#"--
select t.*, t.a from t
"#
    );
}

#[test]
fn format_table_star_normalizes_spacing() {
    assert_fmt_eq!(
        r#"--
select count( t . *  ) from t;
"#,
        r#"--
select count(t.*) from t;
"#
    );
}
