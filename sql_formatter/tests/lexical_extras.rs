#[macro_use]
mod helper;

#[test]
fn format_positional_parameters_and_operators() {
    assert_fmt_eq!(
        r#"--
SELECT $1::NUMERIC, a&&b, X'0000FFFF'::int FROM t WHERE c = $2;
"#,
        r#"--
select $1::numeric, a && b, X'0000FFFF'::int from t where c = $2;
"#
    );
}

#[test]
fn format_continued_string_is_kept_verbatim() {
    assert_fmt_eq!(
        r#"--
select 'from _flds'
    ' join t0 on t0.id = _flds.id' as clause from t;
"#,
        r#"--
select 'from _flds'
    ' join t0 on t0.id = _flds.id' as clause
from t;
"#
    );
}

#[test]
fn format_number_without_leading_digit() {
    assert_fmt_eq!(
        r#"--
SELECT .5, -.25e2, t.a FROM t WHERE x > .1;
"#,
        r#"--
select .5, -.25e2, t.a from t where x > .1;
"#
    );
}
