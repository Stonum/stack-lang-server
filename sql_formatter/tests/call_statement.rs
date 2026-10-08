#[macro_use]
mod helper;

#[test]
fn format_call_without_arguments() {
    assert_fmt!(
        r#"--
call p();
"#
    );
}

#[test]
fn format_call_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CALL   ~SomeProc~( :1,:2 ,  :3::date );
"#,
        r#"--
call ~SomeProc~(:1, :2, :3::date);
"#
    );
}

#[test]
fn format_call_qualified_name() {
    assert_fmt_eq!(
        r#"--
call  s.p(1,'a')
"#,
        r#"--
call s.p(1, 'a')
"#
    );
}
