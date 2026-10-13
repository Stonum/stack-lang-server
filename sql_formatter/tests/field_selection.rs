#[macro_use]
mod helper;

#[test]
fn format_field_selection_keeps_its_parentheses() {
    assert_fmt!(
        r#"--
select (jsonb_each_text(t.d)).key::int as k, (jsonb_each_text(t.d)).value, (t.c).f.g from t
"#
    );
}

#[test]
fn format_field_selection_normalizes_spacing() {
    assert_fmt_eq!(
        r#"--
SELECT ( jsonb_each_text( t.d ) ).key FROM t
"#,
        r#"--
select (jsonb_each_text(t.d)).key from t
"#
    );
}

#[test]
fn format_long_select_with_field_selections_wraps() {
    assert_fmt_eq!(
        r#"--
select (jsonb_each_text(journal_entry.performers)).key::int as performer, (jsonb_each_text(journal_entry.performers)).value as planned from journal_entry
"#,
        r#"--
select
	(jsonb_each_text(journal_entry.performers)).key::int as performer,
	(jsonb_each_text(journal_entry.performers)).value as planned
from journal_entry
"#
    );
}
