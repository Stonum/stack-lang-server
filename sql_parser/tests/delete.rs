#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

/// `DELETE ... USING` is Postgres-only.
fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

#[test]
fn test_delete_from_table() {
    let res = parse("delete from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_delete_from_qualified_table_with_alias() {
    let res = parse("delete from myschema.mytable as t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_delete_where_clause() {
    let res = parse("delete from t where a = 1", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_delete_using_clause() {
    let res = parse("delete from t using u where t.a = u.a", postgres());

    assert_parser!(res);
}

#[test]
fn test_delete_using_function_binding() {
    let res = parse(
        "delete from t using some_func(1) as f where t.a = f.a",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_delete_using_rejected_under_standard_dialect() {
    let res = parse(
        "delete from t using u where t.a = u.a",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_delete_trailing_semicolon() {
    let res = parse("delete from t;", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_multiple_delete_and_select_statements() {
    let res = parse(
        "delete from t where a = 1; select a from t;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}
