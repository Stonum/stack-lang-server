#[macro_use]
mod helper;

#[test]
fn format_array_literal() {
    assert_fmt!(
        r#"--
select array[1, 2, 3] from t
"#
    );
}

#[test]
fn format_array_literal_empty() {
    assert_fmt!(
        r#"--
select array[] from t
"#
    );
}

#[test]
fn format_array_subscript() {
    assert_fmt!(
        r#"--
select a[0] from t
"#
    );
}

#[test]
fn format_array_subscript_chained() {
    assert_fmt!(
        r#"--
select matrix[0][1] from t
"#
    );
}

#[test]
fn format_array_literal_wraps_when_too_long() {
    // PsqlExpressionList (shared by array literals, call arguments, and
    // INSERT...VALUES) uses a fill layout: simple items (bare names,
    // literals) pack multiple per line rather than always one per line --
    // see format_function_call_arguments_wrap_when_too_long below for the
    // case where a *complex* item (its own call expression) forces a line
    // break around itself regardless of how short it is.
    assert_fmt!(
        r#"--
select array[
	really_quite_noticeably_long_element_one, really_quite_noticeably_long_element_two,
	really_quite_noticeably_long_element_three, really_quite_noticeably_long_element_four
]
from t
"#
    );
}

#[test]
fn format_array_cast() {
    assert_fmt!(
        r#"--
select a::int[] from t
"#
    );
}
