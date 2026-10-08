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
fn test_call_without_arguments() {
    let res = parse("call p();", SqlFileSource::script());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_CALL_STATEMENT"));
}

#[test]
fn test_call_qualified_with_arguments() {
    let res = parse("call s.p(1, 'a', x + 1)", postgres());

    assert_parser!(res);
}

#[test]
fn test_call_mlang_tilde_name_and_parameters() {
    let res = parse("call ~SomeProc~(:1, :2, :3::date);", mlang());

    assert_parser!(res);
}

#[test]
fn test_call_followed_by_statement_without_semicolon() {
    let res = parse("call p(1)\nanalyze t", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_ANALYZE_STATEMENT"));
}

#[test]
fn test_call_is_still_a_valid_column_name() {
    let res = parse("select call from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_call_without_parentheses_reports_error() {
    let res = parse("call p;", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_call_without_name_reports_error() {
    let res = parse("call;", postgres());

    assert!(res.has_errors());
}
