#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_interval_literal_addition() {
    let res = parse(
        "select now() + interval '1 day' from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_interval_literal_subtraction() {
    let res = parse(
        "select a from t where dt < now() - interval '1 second'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_interval_as_type_name_still_works() {
    let res = parse("select a::interval from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_interval_as_column_type_still_works() {
    let res = parse("create table t (dur interval)", PsqlFileSource::script());

    assert_parser!(res);
}
