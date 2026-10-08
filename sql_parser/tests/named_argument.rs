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
fn test_named_arguments() {
    let res = parse(
        "select make_interval(hours => 1, mins => x * 2)",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("PSQL_NAMED_ARGUMENT").count(), 2);
    assert!(tree.contains("FAT_ARROW"));
}

#[test]
fn test_positional_and_named_arguments_mixed() {
    let res = parse("select f(1, 'a', flag => true)", postgres());

    assert_parser!(res);
}

#[test]
fn test_named_argument_with_mlang_parameter() {
    let res = parse(
        "select now() - make_interval(days => :1) from ~Таблица~",
        mlang(),
    );

    assert_parser!(res);
}

#[test]
fn test_named_argument_with_keyword_name() {
    let res = parse("select f(key => 1, text => 'a')", postgres());

    assert_parser!(res);
}

#[test]
fn test_named_argument_in_call_statement() {
    let res = parse("call p(a => 1, b => 2);", postgres());

    assert_parser!(res);
}

#[test]
fn test_greater_or_equal_and_equal_still_lex_as_before() {
    let res = parse(
        "select a from t where a >= 1 and b = 2 and c =1",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_named_argument_without_value_reports_error() {
    let res = parse("select f(a =>)", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_named_argument_rejected_under_mssql_dialect() {
    let res = parse(
        "select f(a => 1)",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}
