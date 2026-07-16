#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_cast_operator_on_column() {
    let res = parse("select a::text from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cast_operator_on_literal() {
    let res = parse("select 1::numeric from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_chained_cast_operator() {
    let res = parse("select a::int::text from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cast_operator_with_type_arguments() {
    let res = parse(
        "select price::numeric(10, 2) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_cast_operator_with_array_suffix() {
    let res = parse("select tags::text[] from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cast_operator_on_custom_type() {
    let res = parse("select a::my_enum from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cast_operator_binds_tighter_than_unary_minus() {
    let res = parse("select -1::text from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cast_operator_on_array_subscript() {
    let res = parse("select a[0]::text from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cast_function_expression() {
    let res = parse("select cast(a as text) from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cast_function_expression_with_arguments() {
    let res = parse(
        "select cast(price as numeric(10, 2)) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_cast_operator_in_where_clause() {
    let res = parse("select a from t where a::int = 1", PsqlFileSource::script());

    assert_parser!(res);
}
