#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

#[test]
fn test_function_alias_with_typed_column_list() {
    // The canonical real-world use: a record-returning function's own
    // signature doesn't fix the output columns, so the caller must spell
    // them out. `::jsonb` is Postgres-only.
    let res = parse(
        "select * from json_to_recordset(:1::jsonb) as x(a int, b text)",
        SqlFileSource::script().with_dialect(SqlDialect::Postgres),
    );

    assert_parser!(res);
}

#[test]
fn test_subquery_alias_with_plain_rename_list() {
    // No types here -- just renaming the subquery's own columns.
    let res = parse(
        "select * from (select a, b from t) x(c, d)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_table_alias_with_column_list_no_as() {
    let res = parse("select * from t x(a, b)", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_alias_column_name_colliding_with_a_reserved_keyword() {
    // Real-world confirmed: an alias column is renaming an arbitrary
    // source column, so it collides with this grammar's own reserved
    // words often (e.g. `key`).
    let res = parse(
        "select * from some_func(1) as f(key, val)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_alias_without_column_list_still_works() {
    let res = parse("select * from t as x", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_implicit_alias_without_column_list_still_works() {
    let res = parse("select * from t x", SqlFileSource::script());

    assert_parser!(res);
}
