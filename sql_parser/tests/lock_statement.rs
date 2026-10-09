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
fn test_lock_statement_forms() {
    for src in [
        "lock t",
        "lock table t;",
        "lock table only a, s.b nowait",
        "lock t in access share mode",
        "lock t in row share mode",
        "lock t in row exclusive mode",
        "lock t in share update exclusive mode",
        "lock t in share mode",
        "lock t in share row exclusive mode nowait;",
        "lock t in exclusive mode",
        "LOCK TABLE t IN ACCESS EXCLUSIVE MODE;",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_LOCK_STATEMENT"), "{src}");
    }
}

#[test]
fn test_lock_tilde_table_in_transaction_batch() {
    let res = parse(
        "begin\nlock table ~Таблица~ in share row exclusive mode\ntruncate ~Таблица~\ncommit",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_LOCK_STATEMENT"));
    assert!(tree.contains("SQL_TRUNCATE_STATEMENT"));
    assert!(tree.contains("SQL_COMMIT_STATEMENT"));
}

#[test]
fn test_lock_keywords_are_still_valid_names() {
    let res = parse(
        "select lock, access, share, exclusive, mode, nowait from t",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_lock_invalid_mode_combinations_report_errors() {
    for src in [
        "lock t in access mode",
        "lock t in row update exclusive mode",
        "lock t in share update mode",
        "lock t in exclusive",
        "lock t in mode",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}

#[test]
fn test_lock_without_table_reports_error() {
    let res = parse("lock table;", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_lock_rejected_under_mssql_dialect() {
    let res = parse(
        "lock table t",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}
