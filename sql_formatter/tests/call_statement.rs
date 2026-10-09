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

#[test]
fn format_call_arguments_wrap_when_too_long() {
    assert_fmt_eq!(
        r#"--
call some_schema.some_long_procedure_name(:first_parameter, :second_parameter, 'a long string literal value', :third_parameter::date)
"#,
        r#"--
call some_schema.some_long_procedure_name(
	:first_parameter, :second_parameter, 'a long string literal value',
	:third_parameter::date
)
"#
    );
}

#[test]
fn format_call_many_arguments_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
call p(:a, :b, :c, :d, :e, :f, :g, :h, :i, :j, :k, :l, :m, :n, :o, :p, :q, :r, :s, :t, :u, :v, :w, :x, :y, :z, :aa, :bb, :cc, :dd, :ee, :ff, :gg, :hh, :ii, :jj, :kk, :ll, :mm, :nn, :oo, :pp, :qq, :rr, :ss, :tt, :uu, :vv, :ww, :xx, :yy, :zz)
"#,
        r#"--
call p(
	:a, :b, :c, :d, :e, :f, :g, :h, :i, :j, :k, :l, :m, :n, :o, :p, :q, :r, :s, :t, :u, :v, :w, :x, :y, :z, :aa, :bb, :cc,
	:dd, :ee, :ff, :gg, :hh, :ii, :jj, :kk, :ll, :mm, :nn, :oo, :pp, :qq, :rr, :ss, :tt, :uu, :vv, :ww, :xx, :yy, :zz
)
"#
    );
}
