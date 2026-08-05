#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_from_requires_at_least_one_item() {
    let res = parse("select a from", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_delete_using_requires_at_least_one_item() {
    let res = parse("delete from t using where t.a = 1", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_group_by_requires_at_least_one_item() {
    let res = parse("select a from t group by", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_order_by_requires_at_least_one_item() {
    let res = parse("select a from t order by", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_update_set_requires_at_least_one_item() {
    let res = parse("update t set", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_do_update_set_requires_at_least_one_item() {
    let res = parse(
        "insert into t (id) values (1) on conflict (id) do update set",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_window_partition_by_requires_at_least_one_item() {
    let res = parse(
        "select row_number() over (partition by) from t",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_window_order_by_requires_at_least_one_item() {
    let res = parse(
        "select rank() over (order by) from t",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_call_arguments_still_allow_zero() {
    // Not every list requires at least one element -- call args and array
    // literals are legitimately empty in real Postgres (`now()`,
    // `array[]::int[]`), unlike FROM/GROUP BY/ORDER BY/SET/PARTITION BY.
    let res = parse("select now() from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_array_literal_still_allows_zero() {
    let res = parse("select array[] from t", SqlFileSource::script());

    assert_parser!(res);
}
