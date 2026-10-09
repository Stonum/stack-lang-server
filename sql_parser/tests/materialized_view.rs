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
fn test_create_materialized_view_forms() {
    for src in [
        "create materialized view mv as select a from t",
        "create materialized view if not exists mv (a, b) as select a, b from t with no data;",
        "create materialized view mv with (fillfactor = 70) as select a from t with data",
        "create materialized view mv as (with x as (select 1 as a) select a from x)",
        "create materialized view mv as with x as (select 1 as a) select a from x",
        "create materialized view mv as table t",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(
            tree.contains("PSQL_CREATE_MATERIALIZED_VIEW_STATEMENT"),
            "{src}"
        );
    }
}

#[test]
fn test_drop_and_refresh_materialized_view() {
    let res = parse(
        "drop materialized view if exists ~Таблица~, s.mv cascade;\nrefresh materialized view concurrently mv with data;",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_DROP_MATERIALIZED_VIEW_STATEMENT"));
    assert!(tree.contains("PSQL_REFRESH_MATERIALIZED_VIEW_STATEMENT"));
}

#[test]
fn test_drop_and_create_materialized_view_batch_without_semicolon() {
    let res = parse(
        "drop materialized view if exists mv\ncreate materialized view mv as select 1",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_CREATE_MATERIALIZED_VIEW_STATEMENT"));
}

#[test]
fn test_refresh_is_still_a_valid_column_name() {
    let res = parse("select refresh from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_create_materialized_view_without_query_reports_error() {
    let res = parse("create materialized view mv as;", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_refresh_without_view_reports_error() {
    let res = parse("refresh materialized mv", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_materialized_views_rejected_under_mssql_dialect() {
    for src in [
        "create materialized view mv as select 1",
        "drop materialized view mv",
        "refresh materialized view mv",
    ] {
        let res = parse(src, SqlFileSource::script().with_dialect(SqlDialect::Mssql));

        assert!(res.has_errors(), "{src}");
    }
}
