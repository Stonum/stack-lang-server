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
    assert_fmt!(
        r#"--
select array[
	really_long_element_one,
	really_long_element_two,
	really_long_element_three,
	really_long_element_four
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
