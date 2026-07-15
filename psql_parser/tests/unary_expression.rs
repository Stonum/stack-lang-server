#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_unary_minus_and_plus() {
    let res = parse("select -1, +1, -a, +a from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_unary_minus_binds_tighter_than_binary_operators() {
    let res = parse("select -1 + 2, -1 * 2 from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_double_unary_minus() {
    let res = parse("select - -1 from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_not_expression() {
    let res = parse("select a from t where not a", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_not_binds_tighter_than_and_but_looser_than_comparison() {
    let res = parse(
        "select a from t where not a = 1 and b",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_with_parenthesized_expression() {
    let res = parse(
        "select a from t where not (a and b)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_double_not() {
    let res = parse("select a from t where not not a", PsqlFileSource::script());

    assert_parser!(res);
}
