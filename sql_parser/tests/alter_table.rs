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

#[test]
fn test_alter_table_add_column_if_not_exists() {
    let res = parse(
        r#"alter table #tmp add column if not exists "Поле" numeric(15, 2) not null default 0"#,
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_ADD_COLUMN_ACTION"));
}

#[test]
fn test_alter_table_drop_columns() {
    let res = parse(
        "alter table ~Первая таблица~ drop column if exists a, drop column b cascade, drop c",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_DROP_COLUMN_ACTION").count(), 3);
}

#[test]
fn test_alter_table_add_and_drop_mixed() {
    let res = parse("alter table t add b int, drop column a", postgres());

    assert_parser!(res);
}

#[test]
fn test_alter_table_drop_column_does_not_swallow_next_drop_statement() {
    let res = parse("alter table t drop column a\ndrop table u", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_DROP_TABLE_STATEMENT"));
}

#[test]
fn test_alter_table_add_column_without_definition_reports_error() {
    let res = parse("alter table t add column;", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_column_is_reserved() {
    let res = parse("select column from t", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_alter_column_type_using() {
    let res = parse(
        r#"alter table ~Таблица~ alter column "Поле" type int2 using "Поле"::int2"#,
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_ALTER_COLUMN_TYPE"));
    assert!(tree.contains("PSQL_ALTER_COLUMN_USING"));
}

#[test]
fn test_alter_column_default_and_not_null_operations() {
    let res = parse(
        "alter table t alter column a set default 0, alter b drop default, alter column c set not null, alter column d drop not null, alter e set data type text",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_SET_COLUMN_DEFAULT"));
    assert!(tree.contains("SQL_DROP_COLUMN_DEFAULT"));
    assert!(tree.contains("SQL_SET_COLUMN_NOT_NULL"));
    assert!(tree.contains("SQL_DROP_COLUMN_NOT_NULL"));
    assert_eq!(tree.matches("SQL_ALTER_COLUMN_ACTION").count(), 5);
}

#[test]
fn test_alter_column_does_not_swallow_next_alter_table_statement() {
    let res = parse(
        "alter table t alter column a drop default\nalter table u disable trigger all",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_ALTER_TABLE_STATEMENT").count(), 2);
}

#[test]
fn test_type_is_still_a_valid_column_name() {
    let res = parse("select type from t where type = 1", postgres());

    assert_parser!(res);
}

#[test]
fn test_alter_column_without_operation_reports_error() {
    let res = parse("alter table t alter column a;", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_alter_column_using_rejected_under_mssql_dialect() {
    let res = parse(
        "alter table t alter column a type int using a",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}
