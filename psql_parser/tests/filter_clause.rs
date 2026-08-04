#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_filter_clause_on_aggregate() {
    let res = parse(
        "select count(x) filter (where a > 1) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_filter_clause_multiple_aggregates() {
    let res = parse(
        "select count(x) filter (where a > 1), sum(y) filter (where b < 2) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_filter_clause_with_window_function() {
    let res = parse(
        "select count(x) filter (where a > 1) over (partition by b) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_call_without_filter_clause_still_works() {
    let res = parse("select count(x) from t", PsqlFileSource::script());

    assert_parser!(res);
}
