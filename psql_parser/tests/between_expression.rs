#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_between() {
    let res = parse(
        "select a from t where a between 1 and 10",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_between() {
    let res = parse(
        "select a from t where a not between 1 and 10",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_between_with_arithmetic_bounds() {
    let res = parse(
        "select a from t where a between b + 1 and c - 1",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_between_chained_with_and() {
    let res = parse(
        "select a from t where a between 1 and 10 and b",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_binds_looser_than_between() {
    let res = parse(
        "select a from t where not a between 1 and 10",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
