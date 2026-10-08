#[macro_use]
mod helper;

#[test]
fn format_vacuum_alone() {
    assert_fmt!(
        r#"--
vacuum;
"#
    );
}

#[test]
fn format_analyze_alone() {
    assert_fmt!(
        r#"--
analyze
"#
    );
}

#[test]
fn format_vacuum_flags_and_column_lists() {
    assert_fmt!(
        r#"--
vacuum full freeze verbose analyze a (x, y), s.b;
"#
    );
}

#[test]
fn format_vacuum_analyze_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
VACUUM   ANALYZE   ~Таблица~;
"#,
        r#"--
vacuum analyze ~Таблица~;
"#
    );
}

#[test]
fn format_analyze_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
ANALYZE  VERBOSE   #tmp,b(x,y);
"#,
        r#"--
analyze verbose #tmp, b (x, y);
"#
    );
}
