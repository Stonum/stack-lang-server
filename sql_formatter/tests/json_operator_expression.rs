#[macro_use]
mod helper;

#[test]
fn format_json_field_operator() {
    assert_fmt!(
        r#"--
select data -> 'a' from t
"#
    );
}

#[test]
fn format_json_text_operator() {
    assert_fmt!(
        r#"--
select data ->> 'a' from t
"#
    );
}

#[test]
fn format_chained_json_operators() {
    assert_fmt!(
        r#"--
select data -> 'a' ->> 'b' from t
"#
    );
}

#[test]
fn format_json_operator_normalizes_spacing() {
    assert_fmt_eq!(
        r#"--
select data->'a', data->>'b' from t;
"#,
        r#"--
select data -> 'a', data ->> 'b' from t;
"#
    );
}

#[test]
fn format_json_operator_in_where_clause() {
    assert_fmt!(
        r#"--
select a from t where data -> 'a' = 'x' and data ->> 'id' = '1'
"#
    );
}

#[test]
fn format_jsonb_key_exists_operator() {
    assert_fmt_eq!(
        r#"--
select case when data->'old'?'key' then 1 end from t
"#,
        r#"--
select case when data -> 'old' ? 'key' then 1 end from t
"#
    );
}

#[test]
fn format_long_condition_with_key_exists_operator_wraps() {
    assert_fmt_eq!(
        r#"--
select a from t where journal_entry.data -> 'data_old' ? 'planned_time' and journal_entry.data -> 'data_new' ? 'planned_time' and journal_entry.data ? 'actual_time'
"#,
        r#"--
select a
from t
where journal_entry.data -> 'data_old' ? 'planned_time'
	and journal_entry.data -> 'data_new' ? 'planned_time'
	and journal_entry.data ? 'actual_time'
"#
    );
}
