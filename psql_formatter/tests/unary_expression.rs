#[macro_use]
mod helper;

#[test]
fn format_unary_minus() {
    assert_fmt!(
        r#"--
select -a
"#
    );
}

#[test]
fn format_unary_plus() {
    assert_fmt!(
        r#"--
select +a
"#
    );
}

#[test]
fn format_not() {
    assert_fmt!(
        r#"--
select a from t where not a
"#
    );
}

#[test]
fn format_nested_same_sign_unary_gets_parenthesized() {
    // `- -a` -- printed adjacent without a separator, the two `-` signs
    // would merge into `--`, a line comment starter; `NeedsParentheses`
    // wraps the nested same-sign unary instead of relying on spacing.
    assert_fmt!(
        r#"--
select -(-a)
"#
    );
}

#[test]
fn format_nested_mixed_sign_unary_does_not_need_parens() {
    assert_fmt!(
        r#"--
select -+a
"#
    );
}

#[test]
fn format_nested_not_does_not_need_parens() {
    assert_fmt!(
        r#"--
select not not a
"#
    );
}

#[test]
fn format_unary_normalizes_case() {
    assert_fmt_eq!(
        r#"--
select a from t where NOT a
"#,
        r#"--
select a from t where not a
"#
    );
}
