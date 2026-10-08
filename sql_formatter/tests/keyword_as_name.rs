#[macro_use]
mod helper;

/// A non-reserved keyword used as a name is an identifier: its spelling is
/// kept as written, while real keywords around it are still lowercased.
#[test]
fn format_keyword_named_columns_keep_their_spelling() {
    assert_fmt_eq!(
        r#"SELECT   Key,ROWS,  t.Text FROM T WHERE Rows>0
"#,
        r#"select Key, ROWS, t.Text from T where Rows > 0
"#
    );
}

#[test]
fn format_reserved_keyword_alias_keeps_its_spelling() {
    assert_fmt_eq!(
        r#"select a   AS Table from t
"#,
        r#"select a as Table from t
"#
    );
}
