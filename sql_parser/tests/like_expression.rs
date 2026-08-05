#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_like() {
    let res = parse(
        "select a from t where a like 'foo%'",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_ilike() {
    let res = parse(
        "select a from t where a ilike 'foo%'",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_like() {
    let res = parse(
        "select a from t where a not like 'foo%'",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_ilike() {
    let res = parse(
        "select a from t where a not ilike 'foo%'",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_like_chained_with_and() {
    let res = parse(
        "select a from t where a like 'foo%' and b",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_binds_looser_than_like() {
    let res = parse(
        "select a from t where not a like 'foo%'",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}
