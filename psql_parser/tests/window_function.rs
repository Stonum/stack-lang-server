#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_window_function_empty_spec() {
    let res = parse(
        "select row_number() over () from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_function_partition_by() {
    let res = parse(
        "select sum(a) over (partition by dept) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_function_order_by() {
    let res = parse(
        "select rank() over (order by salary desc) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_function_partition_by_and_order_by() {
    let res = parse(
        "select row_number() over (partition by dept order by salary desc) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_function_multiple_partition_columns() {
    let res = parse(
        "select sum(a) over (partition by dept, region) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_function_multiple_order_by_columns() {
    let res = parse(
        "select rank() over (order by dept asc, salary desc) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_function_with_alias() {
    let res = parse(
        "select row_number() over (partition by dept) as rn from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_function_with_call_arguments() {
    let res = parse(
        "select lag(a, 1) over (order by b) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_function_cast_result() {
    let res = parse(
        "select row_number() over ()::text from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_function_in_where_clause_disallowed_call_still_parses() {
    // Window functions can't actually appear in WHERE in real Postgres, but
    // the parser doesn't enforce that -- just checking it doesn't choke.
    let res = parse(
        "select a from t where sum(a) over (partition by b) > 1",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_multiple_window_functions() {
    let res = parse(
        "select row_number() over (order by a), rank() over (order by b) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
