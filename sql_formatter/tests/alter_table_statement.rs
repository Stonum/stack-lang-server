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
