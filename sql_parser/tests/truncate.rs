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
fn test_truncate_bare_name() {
    let res = parse("truncate t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_truncate_table_with_semicolon() {
    let res = parse("truncate table t;", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_truncate_multiple_qualified_tables() {
    let res = parse("truncate table a, s.b, c", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_truncate_restart_identity_cascade() {
    let res = parse("truncate table a, b restart identity cascade;", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_TRUNCATE_IDENTITY_CLAUSE"));
}

#[test]
fn test_truncate_continue_identity_restrict() {
    let res = parse("truncate t continue identity restrict", postgres());

    assert_parser!(res);
}

#[test]
fn test_truncate_mlang_table_names() {
    let res = parse("truncate table #tmp, ~Таблица~, key", mlang());

    assert_parser!(res);
}

#[test]
fn test_truncate_followed_by_statement_without_semicolon() {
    let res = parse("truncate table a\nselect 1", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_TRUNCATE_STATEMENT"));
    assert!(tree.contains("SQL_SELECT_STATEMENT"));
}

#[test]
fn test_truncate_is_still_a_valid_column_name() {
    let res = parse("select truncate, restart, identity from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_truncate_without_table_reports_error() {
    let res = parse("truncate table;", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_truncate_identity_clause_rejected_under_mssql_dialect() {
    let res = parse(
        "truncate table t restart identity",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}

#[test]
fn test_truncate_cascade_rejected_under_standard_dialect() {
    let res = parse("truncate table t cascade", SqlFileSource::script());

    assert!(res.has_errors());
}
