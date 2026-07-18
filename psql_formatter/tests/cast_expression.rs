#[macro_use]
mod helper;

#[test]
fn format_double_colon_cast() {
    assert_fmt!(
        r#"--
select a::int from t
"#
    );
}

#[test]
fn format_double_colon_cast_with_args() {
    assert_fmt!(
        r#"--
select a::numeric(10, 2) from t
"#
    );
}

#[test]
fn format_double_colon_cast_array_suffix() {
    assert_fmt!(
        r#"--
select a::int[] from t
"#
    );
}

#[test]
fn format_double_colon_cast_tilde_array_suffix() {
    assert_fmt!(
        r#"--
select a::int~[]~ from t
"#
    );
}

#[test]
fn format_cast_function() {
    assert_fmt!(
        r#"--
select cast(a as int) from t
"#
    );
}

#[test]
fn format_cast_function_with_type_args() {
    assert_fmt!(
        r#"--
select cast(a as numeric(10, 2)) from t
"#
    );
}

#[test]
fn format_chained_double_colon_casts() {
    assert_fmt!(
        r#"--
select a::text::int from t
"#
    );
}
