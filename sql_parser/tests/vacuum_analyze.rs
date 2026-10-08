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
fn test_vacuum_alone() {
    let res = parse("vacuum;", postgres());

    assert_parser!(res);
}

#[test]
fn test_vacuum_analyze_table() {
    let res = parse("vacuum analyze t", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_VACUUM_STATEMENT"));
}

#[test]
fn test_vacuum_all_flags_and_column_lists() {
    let res = parse(
        "vacuum full freeze verbose analyse a (x, y), s.b;",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_analyze_alone() {
    let res = parse("analyze", postgres());

    assert_parser!(res);
}

#[test]
fn test_analyze_verbose_tables_with_columns() {
    let res = parse("analyze verbose a, b (x);", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_ANALYZE_STATEMENT"));
}

#[test]
fn test_analyse_spelling() {
    let res = parse("analyse t", postgres());

    assert_parser!(res);
}

#[test]
fn test_vacuum_analyze_mlang_table_names() {
    let res = parse("vacuum analyze ~Таблица~; analyze #tmp", mlang());

    assert_parser!(res);
}

#[test]
fn test_analyze_followed_by_statement_without_semicolon() {
    let res = parse(
        "insert into #t select 1;\nanalyze #t\nselect * from #t",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_ANALYZE_STATEMENT"));
    assert!(tree.contains("SQL_SELECT_STATEMENT"));
}

#[test]
fn test_vacuum_is_still_a_valid_column_name() {
    let res = parse("select vacuum from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_vacuum_rejected_under_mssql_dialect() {
    let res = parse(
        "vacuum analyze t",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}

#[test]
fn test_analyze_rejected_under_standard_dialect() {
    let res = parse("analyze t", SqlFileSource::script());

    assert!(res.has_errors());
}
