#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_is_null() {
    let res = parse("select a from t where a is null", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_is_not_null() {
    let res = parse(
        "select a from t where a is not null",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_is_null_binds_tighter_than_not_and_and() {
    let res = parse(
        "select a from t where not a is null and b",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_is_null_looser_than_comparison() {
    let res = parse(
        "select a from t where a = 1 is null",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_chained_is_null() {
    let res = parse(
        "select a from t where a is null is null",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}
