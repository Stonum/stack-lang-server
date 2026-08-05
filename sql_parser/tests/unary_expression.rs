#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_unary_minus_and_plus() {
    let res = parse("select -1, +1, -a, +a from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_unary_minus_binds_tighter_than_binary_operators() {
    let res = parse("select -1 + 2, -1 * 2 from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_double_unary_minus() {
    let res = parse("select - -1 from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_not_expression() {
    let res = parse("select a from t where not a", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_not_binds_tighter_than_and_but_looser_than_comparison() {
    let res = parse(
        "select a from t where not a = 1 and b",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_with_parenthesized_expression() {
    let res = parse(
        "select a from t where not (a and b)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_double_not() {
    let res = parse("select a from t where not not a", SqlFileSource::script());

    assert_parser!(res);
}
