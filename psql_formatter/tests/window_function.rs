#[macro_use]
mod helper;

#[test]
fn format_call_expression() {
    assert_fmt!(
        r#"--
select count(a) from t
"#
    );
}

#[test]
fn format_call_expression_multiple_args() {
    assert_fmt!(
        r#"--
select coalesce(a, b, 0) from t
"#
    );
}

#[test]
fn format_call_expression_no_args() {
    assert_fmt!(
        r#"--
select now() from t
"#
    );
}

#[test]
fn format_window_function_partition_by_only() {
    assert_fmt!(
        r#"--
select row_number() over (partition_by dept) from t
"#
    );
}

#[test]
fn format_window_function_order_by_only() {
    assert_fmt!(
        r#"--
select row_number() over (order_by salary desc) from t
"#
    );
}

#[test]
fn format_window_function_partition_and_order() {
    assert_fmt!(
        r#"--
select row_number() over (partition_by dept order_by salary desc) from t
"#
    );
}

#[test]
fn format_window_function_empty_over() {
    assert_fmt!(
        r#"--
select sum(a) over () from t
"#
    );
}

#[test]
fn format_call_arguments_wrap_when_too_long() {
    assert_fmt!(
        r#"--
select coalesce(
	really_long_argument_one,
	really_long_argument_two,
	really_long_argument_three,
	really_long_argument_four
)
from t
"#
    );
}
