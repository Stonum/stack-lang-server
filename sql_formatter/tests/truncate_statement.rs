#[macro_use]
mod helper;

#[test]
fn format_truncate_bare_name() {
    assert_fmt!(
        r#"--
truncate t
"#
    );
}

#[test]
fn format_truncate_table_multiple_names() {
    assert_fmt!(
        r#"--
truncate table a, s.b, #tmp;
"#
    );
}

#[test]
fn format_truncate_restart_identity_cascade() {
    assert_fmt!(
        r#"--
truncate table a restart identity cascade;
"#
    );
}

#[test]
fn format_truncate_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
TRUNCATE   TABLE   ~Таблица~,b   CONTINUE    IDENTITY   RESTRICT;
"#,
        r#"--
truncate table ~Таблица~, b continue identity restrict;
"#
    );
}

#[test]
fn format_truncate_names_wrap_when_too_long() {
    assert_fmt_eq!(
        r#"--
truncate table really_long_table_name_number_one, really_long_table_name_number_two, really_long_table_name_number_three, really_long_table_name_number_four
"#,
        r#"--
truncate table
	really_long_table_name_number_one, really_long_table_name_number_two,
	really_long_table_name_number_three, really_long_table_name_number_four
"#
    );
}
