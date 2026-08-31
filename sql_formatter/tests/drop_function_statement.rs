#[macro_use]
mod helper;

#[test]
fn format_drop_function_bare_name() {
    assert_fmt!(
        r#"--
drop function foo
"#
    );
}

#[test]
fn format_drop_procedure_bare_name() {
    assert_fmt!(
        r#"--
drop procedure foo
"#
    );
}

#[test]
fn format_drop_function_if_exists() {
    assert_fmt!(
        r#"--
drop function if exists foo;
"#
    );
}

#[test]
fn format_drop_function_with_typed_parameters() {
    assert_fmt!(
        r#"--
drop function foo(int, text)
"#
    );
}

#[test]
fn format_drop_function_cascade() {
    assert_fmt!(
        r#"--
drop function if exists foo cascade;
"#
    );
}

#[test]
fn format_drop_function_restrict() {
    assert_fmt!(
        r#"--
drop function if exists foo restrict;
"#
    );
}

#[test]
fn format_drop_function_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
DROP   FUNCTION   IF EXISTS   foo   CASCADE;
"#,
        r#"--
drop function if exists foo cascade;
"#
    );
}

#[test]
fn format_drop_function_parameters_wrap_when_too_long() {
    assert_fmt!(
        r#"--
drop function foo(
	really_long_type_name_number_one, really_long_type_name_number_two,
	really_long_type_name_number_three, really_long_type_name_number_four
)
"#
    );
}
