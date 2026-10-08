#[macro_use]
mod helper;

#[test]
fn format_named_arguments() {
    assert_fmt!(
        r#"--
select make_interval(hours => 1, mins => :2)
"#
    );
}

#[test]
fn format_named_arguments_normalizes_spacing() {
    assert_fmt_eq!(
        r#"--
SELECT make_interval( hours=>1,mins   =>   x*2 )
"#,
        r#"--
select make_interval(hours => 1, mins => x * 2)
"#
    );
}
