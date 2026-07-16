#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_from_requires_at_least_one_item() {
    let res = parse("select a from", PsqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_delete_using_requires_at_least_one_item() {
    let res = parse(
        "delete from t using where t.a = 1",
        PsqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_group_by_requires_at_least_one_item() {
    let res = parse("select a from t group_by", PsqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_order_by_requires_at_least_one_item() {
    let res = parse("select a from t order_by", PsqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_update_set_requires_at_least_one_item() {
    let res = parse("update t set", PsqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_do_update_set_requires_at_least_one_item() {
    let res = parse(
        "insert into t (id) values (1) on conflict (id) do update set",
        PsqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_window_partition_by_requires_at_least_one_item() {
    let res = parse(
        "select row_number() over (partition_by) from t",
        PsqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_window_order_by_requires_at_least_one_item() {
    let res = parse(
        "select rank() over (order_by) from t",
        PsqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_call_arguments_still_allow_zero() {
    // Not every list requires at least one element -- call args and array
    // literals are legitimately empty in real Postgres (`now()`,
    // `array[]::int[]`), unlike FROM/GROUP BY/ORDER BY/SET/PARTITION BY.
    let res = parse("select now() from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_array_literal_still_allows_zero() {
    let res = parse("select array[] from t", PsqlFileSource::script());

    assert_parser!(res);
}
