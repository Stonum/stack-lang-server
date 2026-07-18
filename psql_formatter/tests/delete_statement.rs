#[macro_use]
mod helper;

#[test]
fn format_delete_simple() {
    assert_fmt!(
        r#"--
delete from t
"#
    );
}

#[test]
fn format_delete_with_where() {
    assert_fmt!(
        r#"--
delete from t
where a = 1
"#
    );
}

#[test]
fn format_delete_with_using() {
    assert_fmt!(
        r#"--
delete from t
using t2
where t.id = t2.id
"#
    );
}

#[test]
fn format_delete_with_returning() {
    assert_fmt!(
        r#"--
delete from t
where a = 1
returning a
"#
    );
}

#[test]
fn format_delete_using_multiple_tables_wraps_when_too_long() {
    assert_fmt!(
        r#"--
delete from t
using
	really_long_table_name_one,
	really_long_table_name_two,
	really_long_table_name_three,
	really_long_table_name_four
"#
    );
}
