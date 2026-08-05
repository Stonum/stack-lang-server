#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_call_expression_in_select_list() {
    let res = parse("select count(a), upper(b) from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_call_expression_no_args() {
    let res = parse("select count() from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_qualified_call_expression() {
    let res = parse(
        "select myschema.myfunc(a, b) from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_call_expression_in_where_clause() {
    let res = parse(
        "select a from t where upper(a) = 'X'",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_nested_call_expression() {
    let res = parse(
        "select coalesce(a, upper(b)) from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_call_expression_with_alias() {
    let res = parse("select count(a) as cnt from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_call_expression_star_arg() {
    let res = parse("select count(*) from t", SqlFileSource::script());

    assert_parser!(res);
}
