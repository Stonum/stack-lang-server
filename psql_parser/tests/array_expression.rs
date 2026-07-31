#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_array_literal() {
    let res = parse("select array[1, 2, 3] from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_empty_array_literal() {
    let res = parse("select array[] from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_array_subscript_on_column() {
    let res = parse("select a[1] from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_array_subscript_on_array_literal() {
    let res = parse("select array[1, 2, 3][1] from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_chained_array_subscript() {
    let res = parse("select matrix[0][1] from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_array_subscript_binds_tighter_than_binary_operators() {
    let res = parse("select a[0] + b[0] from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_array_subscript_on_call_expression() {
    let res = parse("select get_array()[0] from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_array_subscript_with_expression_index() {
    let res = parse("select a[i + 1] from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_array_of_expressions() {
    let res = parse(
        "select array[a + 1, upper(b)] from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_tilde_empty_array_literal_in_mlang_dialect() {
    let res = parse("select array~[]~ from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_array_literal_with_elements_in_mlang_dialect() {
    let res = parse("select array~[1, 2, 3]~ from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_array_literal_cast_to_tilde_array_type() {
    // Real-world shape: an empty tilde-escaped array literal cast to a
    // tilde-escaped array type, e.g. a default value for a text[] parameter.
    let res = parse("select array~[]~::text~[]~ from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_array_literal_not_available_outside_mlang_dialect() {
    let res = parse("select array~[]~ from t", PsqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_create_function_parameter_default_tilde_array() {
    let res = parse(
        "create function foo(a text~[]~ default array~[]~::text~[]~) as 'select 1'",
        mlang(),
    );

    assert_parser!(res);
}
