#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

fn mlang() -> SqlFileSource {
    postgres().with_mlang_extension(true)
}

#[test]
fn test_alter_table_disable_trigger_user() {
    let res = parse("alter table ~Таблица~ disable trigger user;", mlang());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_ALTER_TABLE_STATEMENT"));
    assert!(tree.contains("SQL_ALL_TRIGGERS_TARGET"));
}

#[test]
fn test_alter_table_enable_trigger_all_and_named() {
    let res = parse(
        r#"alter table if exists only s.t enable trigger all, disable trigger "t_trigger""#,
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_ALTER_TRIGGER_ACTION").count(), 2);
}

#[test]
fn test_alter_table_batch_without_semicolons() {
    let res = parse(
        "alter table a disable trigger user\nupdate a set x = 1;\nalter table a enable trigger user",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_ALTER_TABLE_STATEMENT").count(), 2);
}

#[test]
fn test_user_and_enable_are_still_valid_column_names() {
    let res = parse("select user, enable, disable from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_alter_table_without_action_reports_error() {
    let res = parse("alter table t;", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_alter_table_trigger_without_target_reports_error() {
    let res = parse("alter table t disable trigger;", postgres());

    assert!(res.has_errors());
}
