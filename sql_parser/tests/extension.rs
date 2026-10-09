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
fn test_create_extension_forms() {
    for src in [
        "create extension hstore",
        "create extension if not exists \"uuid-ossp\" with schema pg_catalog;",
        "create extension pg_trgm schema s cascade",
        "CREATE EXTENSION IF NOT EXISTS \"pgcrypto\";",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_CREATE_EXTENSION_STATEMENT"), "{src}");
    }
}

#[test]
fn test_drop_extension() {
    let res = parse("drop extension if exists a, \"b-c\" cascade;", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_DROP_EXTENSION_STATEMENT"));
}

#[test]
fn test_create_extension_batch_without_semicolon() {
    let res = parse(
        "create extension if not exists a with schema s\ncreate extension b\ndrop extension c",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("PSQL_CREATE_EXTENSION_STATEMENT").count(), 2);
    assert!(tree.contains("PSQL_DROP_EXTENSION_STATEMENT"));
}

#[test]
fn test_extension_is_still_a_valid_column_name() {
    let res = parse("select extension from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_create_extension_without_name_reports_error() {
    let res = parse("create extension if not exists;", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_extension_with_without_schema_reports_error() {
    let res = parse("create extension a with s", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_extensions_rejected_under_mssql_dialect() {
    for src in ["create extension a", "drop extension a"] {
        let res = parse(src, SqlFileSource::script().with_dialect(SqlDialect::Mssql));

        assert!(res.has_errors(), "{src}");
    }
}
