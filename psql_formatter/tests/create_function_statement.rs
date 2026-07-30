#[macro_use]
mod helper;

#[test]
fn format_create_function_no_params() {
    assert_fmt!(
        r#"--
create function foo() as 'select 1'
"#
    );
}

#[test]
fn format_create_procedure_no_params() {
    assert_fmt!(
        r#"--
create procedure foo() as 'select 1'
"#
    );
}

#[test]
fn format_create_function_with_parameters() {
    assert_fmt!(
        r#"--
create function foo(a int, b text) as 'select 1'
"#
    );
}

#[test]
fn format_create_function_returns_scalar() {
    assert_fmt!(
        r#"--
create function foo(a int) returns boolean as 'select 1'
"#
    );
}

#[test]
fn format_create_function_with_language() {
    assert_fmt!(
        r#"--
create function foo(a int) returns boolean as 'select 1' language plpgsql;
"#
    );
}

#[test]
fn format_create_function_dollar_quoted_body_is_preserved_verbatim() {
    // The body is never reformatted, whatever whitespace/indentation it
    // already has -- this is exactly the point of treating it as an opaque
    // string-literal token rather than parsing PL/pgSQL.
    assert_fmt!(
        r#"--
create function foo(a int) returns boolean as $$
begin
  return true;
end;
$$ language plpgsql;
"#
    );
}

#[test]
fn format_create_function_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE   FUNCTION   foo(a   int,b text)   RETURNS BOOLEAN   AS 'select 1'   LANGUAGE   plpgsql;
"#,
        r#"--
create function foo(a int, b text) returns boolean as 'select 1' language plpgsql;
"#
    );
}

#[test]
fn format_create_function_parameter_mode() {
    assert_fmt!(
        r#"--
create function foo(in a int, out b int) as 'select 1'
"#
    );
}

#[test]
fn format_create_function_parameter_default() {
    assert_fmt!(
        r#"--
create function foo(a text default '') as 'select 1'
"#
    );
}

#[test]
fn format_create_function_parameter_mode_and_default_combined() {
    assert_fmt!(
        r#"--
create function foo(in a varchar default ''::varchar) as 'select 1'
"#
    );
}

#[test]
fn format_create_function_returns_trigger() {
    assert_fmt!(
        r#"--
create function foo() returns trigger as 'select 1'
"#
    );
}

#[test]
fn format_create_function_returns_trigger_normalizes_case() {
    assert_fmt_eq!(
        r#"--
create function foo() RETURNS TRIGGER as 'select 1'
"#,
        r#"--
create function foo() returns trigger as 'select 1'
"#
    );
}

#[test]
fn format_create_function_returns_table_single_column() {
    assert_fmt!(
        r#"--
create function foo() returns table(a int) as 'select 1'
"#
    );
}

#[test]
fn format_create_function_returns_table_multiple_columns() {
    assert_fmt!(
        r#"--
create function foo() returns table(a int, b text) as 'select 1'
"#
    );
}

#[test]
fn format_create_function_returns_table_normalizes_spacing() {
    assert_fmt_eq!(
        r#"--
create function foo() returns   TABLE(a   int,b text) as 'select 1'
"#,
        r#"--
create function foo() returns table(a int, b text) as 'select 1'
"#
    );
}

#[test]
fn format_create_function_returns_table_columns_wrap_when_too_long() {
    assert_fmt!(
        r#"--
create function foo() returns table(
	really_long_column_name_number_one int,
	really_long_column_name_number_two text,
	really_long_column_name_number_three boolean,
	really_long_column_name_number_four numeric
) as 'select 1'
"#
    );
}

#[test]
fn format_create_function_parameters_wrap_when_too_long() {
    assert_fmt!(
        r#"--
create function foo(
	really_long_parameter_name_number_one int,
	really_long_parameter_name_number_two text,
	really_long_parameter_name_number_three boolean,
	really_long_parameter_name_number_four numeric
) as 'select 1'
"#
    );
}
