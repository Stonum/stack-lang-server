#[macro_use]
mod helper;

#[test]
fn format_function_alias_with_typed_column_list() {
    assert_fmt!(
        r#"--
select * from json_to_recordset(:1::jsonb) as x(a int, b text)
"#
    );
}

#[test]
fn format_subquery_alias_with_plain_rename_list() {
    assert_fmt!(
        r#"--
select * from (select a, b from t) x(c, d)
"#
    );
}

#[test]
fn format_table_alias_with_column_list_no_as() {
    assert_fmt!(
        r#"--
select * from t x(a, b)
"#
    );
}

#[test]
fn format_alias_column_name_colliding_with_a_reserved_keyword() {
    assert_fmt!(
        r#"--
select * from some_func(1) as f(key, val)
"#
    );
}

#[test]
fn format_alias_column_list_wraps_when_too_long() {
    assert_fmt!(
        r#"--
select *
from json_to_recordset(:1::jsonb) as x(
	really_long_column_name_one int, really_long_column_name_two text,
	really_long_column_name_three boolean, really_long_column_name_four numeric
)
"#
    );
}

#[test]
fn format_alias_column_list_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
select * from t AS x  (  a  int ,b  TEXT );
"#,
        r#"--
select * from t as x(a int, b text);
"#
    );
}
