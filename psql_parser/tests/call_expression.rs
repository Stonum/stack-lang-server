#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_call_expression_in_select_list() {
    let res = parse("select count(a), upper(b) from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_call_expression_no_args() {
    let res = parse("select count() from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_qualified_call_expression() {
    let res = parse(
        "select myschema.myfunc(a, b) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_call_expression_in_where_clause() {
    let res = parse(
        "select a from t where upper(a) = 'X'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_nested_call_expression() {
    let res = parse(
        "select coalesce(a, upper(b)) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_call_expression_with_alias() {
    let res = parse("select count(a) as cnt from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_call_expression_star_arg() {
    let res = parse("select count(*) from t", PsqlFileSource::script());

    assert_parser!(res);
}
