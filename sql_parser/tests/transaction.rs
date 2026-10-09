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
fn test_transaction_statement_forms() {
    for (src, kind) in [
        ("begin", "SQL_BEGIN_STATEMENT"),
        ("BEGIN TRANSACTION;", "SQL_BEGIN_STATEMENT"),
        ("begin work", "SQL_BEGIN_STATEMENT"),
        ("start transaction;", "SQL_BEGIN_STATEMENT"),
        ("commit", "SQL_COMMIT_STATEMENT"),
        ("COMMIT TRANSACTION;", "SQL_COMMIT_STATEMENT"),
        ("commit work", "SQL_COMMIT_STATEMENT"),
        ("rollback;", "SQL_ROLLBACK_STATEMENT"),
        ("ROLLBACK TRANSACTION;", "SQL_ROLLBACK_STATEMENT"),
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains(kind), "{src}");
    }
}

#[test]
fn test_transaction_around_ddl_batch() {
    let res = parse(
        r#"/*'multiplecommand'*/
BEGIN TRANSACTION;
ALTER TABLE ~Таблица~ DISABLE TRIGGER ALL;
UPDATE ~Таблица~ SET "Поле" = 1 WHERE "Поле" IS NULL;
ALTER TABLE ~Таблица~ ENABLE TRIGGER ALL;
COMMIT;"#,
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_BEGIN_STATEMENT"));
    assert!(tree.contains("SQL_COMMIT_STATEMENT"));
}

#[test]
fn test_transaction_batch_without_semicolons() {
    let res = parse(
        "begin transaction\ntruncate t\ncommit\nbegin\nrollback",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_BEGIN_STATEMENT").count(), 2);
    assert!(tree.contains("SQL_TRUNCATE_STATEMENT"));
    assert!(tree.contains("SQL_ROLLBACK_STATEMENT"));
}

#[test]
fn test_transaction_keywords_are_still_valid_names() {
    let res = parse("select transaction, work from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_start_without_transaction_reports_error() {
    let res = parse("start", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_commit_with_unexpected_tail_reports_error() {
    let res = parse("commit everything", postgres());

    assert!(res.has_errors());
}
