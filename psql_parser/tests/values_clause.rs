#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_standalone_values_statement() {
    let res = parse("values (1, 2), (3, 4)", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_standalone_values_single_row() {
    let res = parse("values (1, 2)", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_with_prefixed_standalone_values() {
    let res = parse(
        "with x as (select 1) values (1, 2)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_values_as_cte_body() {
    let res = parse(
        "with cte as (values (1, 2), (3, 4)) select * from cte",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_values_as_cte_body_with_column_list() {
    let res = parse(
        "with cte(a, b) as (values (1, 2), (3, 4)) select * from cte",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_values_as_derived_table() {
    let res = parse(
        "select * from (values (1, 2), (3, 4)) as v(a, b)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_values_as_scalar_subquery() {
    let res = parse("select (values (1))", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_insert_values_multiple_rows() {
    let res = parse(
        "insert into t values (1, 2), (3, 4)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_values_single_row_still_works() {
    let res = parse("insert into t values (1, 2)", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_insert_values_multiple_rows_with_returning() {
    let res = parse(
        "insert into t (a, b) values (1, 2), (3, 4) returning a",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_values_multiple_rows_with_on_conflict() {
    let res = parse(
        "insert into t (a, b) values (1, 2), (3, 4) on conflict (a) do nothing",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_still_works_as_cte_body() {
    // Regression guard: the `select`/`values` dispatch at the CTE-body and
    // subquery-body sites must still take the `select` branch for an
    // ordinary select.
    let res = parse(
        "with cte as (select a from t) select * from cte",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
