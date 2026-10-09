#[macro_use]
mod helper;

#[test]
fn format_copy_statements() {
    assert_fmt!(
        r#"--
copy t (a, b) from '/tmp/data.txt' with (format text, delimiter E'\b', null '')
copy (select a, b from t where a > 0) to stdout (format csv, header)
copy t to program 'gzip > /tmp/t.gz';
"#
    );
}

#[test]
fn format_copy_normalizes_spacing_and_keyword_case() {
    assert_fmt_eq!(
        r#"--
COPY "t" FROM STDIN
(
format csv,
header,
delimiter ';'
);
"#,
        r#"--
copy "t" from stdin (format csv, header, delimiter ';');
"#
    );
}

#[test]
fn format_copy_long_query_wraps() {
    assert_fmt_eq!(
        r#"--
copy (select created, action, table_name, record_id, entity_table, entity_record_id, payload, user_name from journal where created > now() - interval '1 day') to stdout (format csv, header, delimiter ';', encoding 'UTF8')
"#,
        r#"--
copy (
	select created, action, table_name, record_id, entity_table, entity_record_id, payload, user_name
	from journal
	where created > now() - interval '1 day'
) to stdout (format csv, header, delimiter ';', encoding 'UTF8')
"#
    );
}

#[test]
fn format_copy_many_options_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
copy accounts_history_archive from '/var/lib/import/accounts_history_archive.csv' (format csv, header, delimiter ';', quote '"', escape '"', null '', encoding 'UTF8', on_error ignore, log_verbosity verbose)
"#,
        r#"--
copy accounts_history_archive from '/var/lib/import/accounts_history_archive.csv' (
	format csv, header, delimiter ';', quote '"', escape '"',
	null '', encoding 'UTF8', on_error ignore, log_verbosity verbose
)
"#
    );
}
