#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

#[test]
fn test_cast_character_varying() {
    let res = parse(
        "select a::character varying from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_cast_bit_varying() {
    let res = parse("select a::bit varying from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cast_double_precision() {
    let res = parse("select a::double precision from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cast_timestamp_without_time_zone() {
    let res = parse(
        "select a::timestamp without time zone from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_cast_timestamp_with_time_zone() {
    let res = parse(
        "select a::timestamp with time zone from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_cast_time_without_time_zone() {
    let res = parse(
        "select a::time without time zone from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_parameter_character_varying_with_default() {
    // Real-world shape: `character varying` parameter defaulted to an
    // empty string cast to its own type.
    let res = parse(
        "create function foo(a character varying default ''::character varying) as 'select 1'",
        SqlFileSource::script().with_dialect(SqlDialect::Postgres),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_column_timestamp_without_time_zone() {
    let res = parse(
        "create table foo (a timestamp without time zone)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_bare_types_still_work_without_modifier() {
    let res = parse(
        "select a::varchar, b::time, c::timestamp, d::double from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}
