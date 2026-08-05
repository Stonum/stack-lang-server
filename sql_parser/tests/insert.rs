#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_insert_values() {
    let res = parse(
        "insert into t values (1, 'a', true)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_with_columns() {
    let res = parse(
        "insert into t (a, b, c) values (1, 2, 3)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_qualified_table_with_alias() {
    let res = parse(
        "insert into myschema.mytable as t (a) values (1)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_trailing_semicolon() {
    let res = parse("insert into t values (1);", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_multiple_insert_and_update_statements() {
    let res = parse(
        "insert into t values (1); update t set a = 2;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_select() {
    let res = parse(
        "insert into t select a, b from u where a > 1",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_select_with_columns_and_trailing_semicolon() {
    let res = parse(
        "insert into t (a, b) select a, b from u;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_multiple_insert_select_statements() {
    let res = parse(
        "insert into t select a from u; insert into t values (1);",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_parenthesized_select_source() {
    // Real-world confirmed: redundant parens around the `SELECT` source,
    // always alongside an explicit column list.
    let res = parse(
        "insert into t (a, b) (select a, b from u)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_parenthesized_select_distinct_source() {
    let res = parse(
        "insert into t (a, b) (select distinct a, b from u)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}
