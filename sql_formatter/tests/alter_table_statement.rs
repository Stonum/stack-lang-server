#[macro_use]
mod helper;

#[test]
fn format_alter_table_disable_trigger_user() {
    assert_fmt!(
        r#"--
alter table ~Таблица~ disable trigger user;
"#
    );
}

#[test]
fn format_alter_table_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
ALTER   TABLE  IF EXISTS t   ENABLE  TRIGGER  ALL ,DISABLE TRIGGER "Trg";
"#,
        r#"--
alter table if exists t enable trigger all, disable trigger "Trg";
"#
    );
}

#[test]
fn format_alter_table_add_and_drop_columns() {
    assert_fmt!(
        r#"--
alter table t add column if not exists a int not null default 0, drop column if exists b cascade;
"#
    );
}

#[test]
fn format_alter_table_add_column_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
ALTER TABLE t  ADD   COLUMN  a  varchar(10)  NULL ,DROP  b
"#,
        r#"--
alter table t add column a varchar(10) null, drop b
"#
    );
}
