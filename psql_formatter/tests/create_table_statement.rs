#[macro_use]
mod helper;

#[test]
fn format_create_table_single_column() {
    assert_fmt!(
        r#"--
create table foo (a int)
"#
    );
}

#[test]
fn format_create_table_multiple_columns() {
    assert_fmt!(
        r#"--
create table foo (a int, b text, c boolean)
"#
    );
}

#[test]
fn format_create_table_if_not_exists() {
    assert_fmt!(
        r#"--
create table if not exists foo (a int);
"#
    );
}

#[test]
fn format_create_table_qualified_name() {
    assert_fmt!(
        r#"--
create table myschema.foo (a int)
"#
    );
}

#[test]
fn format_create_table_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE   TABLE   IF NOT EXISTS   foo (a   int,b text);
"#,
        r#"--
create table if not exists foo (a int, b text);
"#
    );
}

#[test]
fn format_create_table_columns_wrap_when_too_long() {
    assert_fmt!(
        r#"--
create table foo (
	really_long_column_name_number_one int,
	really_long_column_name_number_two text,
	really_long_column_name_number_three boolean,
	really_long_column_name_number_four numeric
)
"#
    );
}
