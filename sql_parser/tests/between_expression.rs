#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_between() {
    let res = parse(
        "select a from t where a between 1 and 10",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_between() {
    let res = parse(
        "select a from t where a not between 1 and 10",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_between_with_arithmetic_bounds() {
    let res = parse(
        "select a from t where a between b + 1 and c - 1",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_between_chained_with_and() {
    let res = parse(
        "select a from t where a between 1 and 10 and b",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_binds_looser_than_between() {
    let res = parse(
        "select a from t where not a between 1 and 10",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}
