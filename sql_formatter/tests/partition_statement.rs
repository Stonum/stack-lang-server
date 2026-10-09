#[macro_use]
mod helper;

#[test]
fn format_partition_statements() {
    assert_fmt!(
        r#"--
create table t (like s.source) partition by range ("Дата");
create table if not exists s."p_def" partition of ~Таблица~ default;
create table p1 partition of t for values from ('2024-01-01'::date) to ('2024-02-01'::date);
create table p2 partition of t for values from (minvalue) to (100) partition by list (kind);
create table h0 partition of t for values with (modulus 4, remainder 0);
alter table ~Таблица~ attach partition s."p" for values in ('x');
alter table t detach partition p1 concurrently;
"#
    );
}

#[test]
fn format_partition_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE TABLE p PARTITION OF t FOR VALUES WITH ( MODULUS 4 , REMAINDER 1 );
ALTER TABLE t ATTACH PARTITION p DEFAULT;
CREATE TABLE t ( a int ) PARTITION BY LIST ( a );
"#,
        r#"--
create table p partition of t for values with (modulus 4, remainder 1);
alter table t attach partition p default;
create table t (a int) partition by list (a);
"#
    );
}

#[test]
fn format_create_table_partition_of_long_header_wraps() {
    assert_fmt_eq!(
        r#"--
create table accounts_history_2024_january partition of accounts_history for values from ('2024-01-01'::date) to ('2024-02-01'::date) partition by list (account_kind);
"#,
        r#"--
create table accounts_history_2024_january partition of accounts_history
for values from ('2024-01-01'::date) to ('2024-02-01'::date)
partition by list (account_kind);
"#
    );
}

#[test]
fn format_create_table_partition_by_after_wrapped_columns() {
    assert_fmt_eq!(
        r#"--
create table accounts_history (id integer not null, owner integer not null, amount numeric(18, 2), created timestamp not null) partition by range (created, owner);
"#,
        r#"--
create table accounts_history (
	id integer not null,
	owner integer not null,
	amount numeric(18, 2),
	created timestamp not null
) partition by range (created, owner);
"#
    );
}

#[test]
fn format_partition_many_values_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
create table p partition of t for values in ('aa', 'ab', 'ac', 'ad', 'ae', 'af', 'ag', 'ah', 'ai', 'aj', 'ak', 'al', 'am', 'an', 'ao', 'ap', 'aq', 'ar', 'as', 'at', 'au', 'av', 'aw', 'ax');
"#,
        r#"--
create table p partition of t
for values in (
	'aa', 'ab', 'ac', 'ad', 'ae', 'af', 'ag', 'ah', 'ai', 'aj', 'ak', 'al',
	'am', 'an', 'ao', 'ap', 'aq', 'ar', 'as', 'at', 'au', 'av', 'aw', 'ax'
);
"#
    );
}
