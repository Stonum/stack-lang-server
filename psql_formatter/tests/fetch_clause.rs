#[macro_use]
mod helper;

#[test]
fn format_fetch_first_rows_only() {
    assert_fmt!(
        r#"--
select a from t fetch first 5 rows only
"#
    );
}

#[test]
fn format_fetch_first_row_no_count() {
    assert_fmt!(
        r#"--
select a from t fetch first row only
"#
    );
}

#[test]
fn format_fetch_with_ties() {
    assert_fmt!(
        r#"--
select a from t order by a fetch first 1 rows with ties
"#
    );
}

#[test]
fn format_fetch_after_offset() {
    assert_fmt!(
        r#"--
select a from t offset 5 fetch next 10 rows only
"#
    );
}

#[test]
fn format_fetch_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
select a from t FETCH   FIRST   1   ROW   ONLY;
"#,
        r#"--
select a from t fetch first 1 row only;
"#
    );
}
