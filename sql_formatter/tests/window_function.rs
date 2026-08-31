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
select row_number() over (partition by dept) from t
"#
    );
}

#[test]
fn format_window_function_order_by_only() {
    assert_fmt!(
        r#"--
select row_number() over (order by salary desc) from t
"#
    );
}

#[test]
fn format_window_function_partition_and_order() {
    assert_fmt!(
        r#"--
select row_number() over (partition by dept order by salary desc) from t
"#
    );
}

#[test]
fn format_window_function_normalizes_partition_by_order_by_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
select row_number() over (PARTITION   BY dept ORDER by salary desc) from t
"#,
        r#"--
select row_number() over (partition by dept order by salary desc) from t
"#
    );
}

#[test]
fn format_window_function_partition_by_multiple_columns_packs_when_it_fits() {
    assert_fmt!(
        r#"--
select row_number() over (partition by dept, region, team) from t
"#
    );
}

#[test]
fn format_window_function_partition_by_wraps_when_too_long() {
    assert_fmt!(
        r#"--
select row_number() over (partition by
	really_long_column_name_one, really_long_column_name_two, really_long_column_name_three,
	really_long_column_name_four, really_long_column_name_five)
from t
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
	really_long_argument_one, really_long_argument_two, really_long_argument_three, really_long_argument_four
)
from t
"#
    );
}
