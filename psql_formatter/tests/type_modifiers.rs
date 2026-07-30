#[macro_use]
mod helper;

#[test]
fn format_cast_character_varying() {
    assert_fmt!(
        r#"--
select a::character varying from t
"#
    );
}

#[test]
fn format_cast_double_precision() {
    assert_fmt!(
        r#"--
select a::double precision from t
"#
    );
}

#[test]
fn format_cast_timestamp_without_time_zone() {
    assert_fmt!(
        r#"--
select a::timestamp without time zone from t
"#
    );
}

#[test]
fn format_cast_timestamp_with_time_zone() {
    assert_fmt!(
        r#"--
select a::timestamp with time zone from t
"#
    );
}

#[test]
fn format_type_modifiers_normalize_case_and_spacing() {
    assert_fmt_eq!(
        r#"--
select a::TIMESTAMP   WITHOUT   TIME   ZONE from t
"#,
        r#"--
select a::timestamp without time zone from t
"#
    );
}

#[test]
fn format_create_table_column_timestamp_without_time_zone() {
    assert_fmt!(
        r#"--
create table foo (a timestamp without time zone)
"#
    );
}
