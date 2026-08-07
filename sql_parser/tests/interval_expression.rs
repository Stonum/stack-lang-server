#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

/// `INTERVAL '...'` literals and the `interval` type keyword are both
/// Postgres-only.
fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

#[test]
fn test_interval_literal_addition() {
    let res = parse("select now() + interval '1 day' from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_interval_literal_subtraction() {
    let res = parse(
        "select a from t where dt < now() - interval '1 second'",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_interval_as_type_name_still_works() {
    let res = parse("select a::interval from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_interval_as_column_type_still_works() {
    let res = parse("create table t (dur interval)", postgres());

    assert_parser!(res);
}
