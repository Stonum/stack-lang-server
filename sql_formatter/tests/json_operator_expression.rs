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

#[test]
fn format_jsonb_operators_normalize_spacing() {
    assert_fmt_eq!(
        r#"--
select d?|array['a'], d?&array['a'], d@>'{}', '{}'<@d, d#>'{a}', d#>>'{a}', d#-'{a}' from t
"#,
        r#"--
select d ?| array['a'], d ?& array['a'], d @> '{}', '{}' <@ d, d #> '{a}', d #>> '{a}', d #- '{a}' from t
"#
    );
}

#[test]
fn format_long_condition_with_jsonb_containment_wraps() {
    assert_fmt_eq!(
        r#"--
select a from t where journal_entry.payload @> '{"kind": "payment"}' and journal_entry.payload #>> '{owner,code}' = 'first_owner' and journal_entry.tags ?| array['urgent', 'manual']
"#,
        r#"--
select a
from t
where journal_entry.payload @> '{"kind": "payment"}'
	and journal_entry.payload #>> '{owner,code}' = 'first_owner'
	and journal_entry.tags ?| array['urgent', 'manual']
"#
    );
}
