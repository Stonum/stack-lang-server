#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_like() {
    let res = parse(
        "select a from t where a like 'foo%'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_ilike() {
    let res = parse(
        "select a from t where a ilike 'foo%'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_like() {
    let res = parse(
        "select a from t where a not like 'foo%'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_ilike() {
    let res = parse(
        "select a from t where a not ilike 'foo%'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_like_chained_with_and() {
    let res = parse(
        "select a from t where a like 'foo%' and b",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_binds_looser_than_like() {
    let res = parse(
        "select a from t where not a like 'foo%'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
