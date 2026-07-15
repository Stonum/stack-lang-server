#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_delete_from_table() {
    let res = parse("delete from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_delete_from_qualified_table_with_alias() {
    let res = parse(
        "delete from myschema.mytable as t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_delete_where_clause() {
    let res = parse("delete from t where a = 1", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_delete_using_clause() {
    let res = parse(
        "delete from t using u where t.a = u.a",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_delete_using_function_binding() {
    let res = parse(
        "delete from t using some_func(1) as f where t.a = f.a",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_delete_trailing_semicolon() {
    let res = parse("delete from t;", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_multiple_delete_and_select_statements() {
    let res = parse(
        "delete from t where a = 1; select a from t;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
