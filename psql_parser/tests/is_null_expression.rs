#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_is_null() {
    let res = parse("select a from t where a is null", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_is_not_null() {
    let res = parse(
        "select a from t where a is not null",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_is_null_binds_tighter_than_not_and_and() {
    let res = parse(
        "select a from t where not a is null and b",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_is_null_looser_than_comparison() {
    let res = parse(
        "select a from t where a = 1 is null",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_chained_is_null() {
    let res = parse(
        "select a from t where a is null is null",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
