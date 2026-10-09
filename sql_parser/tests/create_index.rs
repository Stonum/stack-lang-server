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

fn mssql() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Mssql)
}

#[test]
fn test_create_index_named() {
    let res = parse("create index ix_t on t (a, b);", SqlFileSource::script());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_CREATE_INDEX_STATEMENT"));
}

#[test]
fn test_create_unique_index_with_every_clause() {
    let res = parse(
        "create unique index concurrently if not exists ix_t on only t using btree (a, b desc) include (c, d) where e > 0",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_INDEX_METHOD_CLAUSE"));
    assert!(tree.contains("SQL_INDEX_INCLUDE_CLAUSE"));
    assert!(tree.contains("SQL_WHERE_CLAUSE"));
}

#[test]
fn test_create_index_without_name_on_mlang_tables() {
    for src in [
        "create index on #tmp (a)",
        "create index if not exists ix_t on ~Таблица~ using btree (\"Поле\")",
    ] {
        let res = parse(src, mlang());

        assert_parser!(res);
    }
}

#[test]
fn test_create_index_expression_elements() {
    let res = parse(
        "create index ix_t on t (lower(name), (amount * 2) desc, coalesce(code, ''))",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_ORDER_BY_EXPRESSION@").count(), 3);
}

#[test]
fn test_create_index_followed_by_statement_without_semicolon() {
    let res = parse("create index on t (a)\nanalyze t", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_ANALYZE_STATEMENT"));
}

#[test]
fn test_include_is_still_a_valid_column_name() {
    let res = parse("select include from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_create_index_without_on_reports_error() {
    let res = parse("create index ix_t (a)", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_index_without_elements_reports_error() {
    let res = parse("create index ix_t on t ()", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_named_index_is_accepted_under_mssql_dialect() {
    let res = parse("create index ix_t on t (a)", mssql());

    assert_parser!(res);
}

#[test]
fn test_create_index_postgres_parts_rejected_under_mssql_dialect() {
    for src in [
        "create index on t (a)",
        "create index ix_t on t using btree (a)",
        "create index concurrently ix_t on t (a)",
        "create index if not exists ix_t on t (a)",
    ] {
        let res = parse(src, mssql());

        assert!(res.has_errors(), "{src}");
    }
}
