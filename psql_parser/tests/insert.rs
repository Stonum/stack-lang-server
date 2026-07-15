#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_insert_values() {
    let res = parse(
        "insert into t values (1, 'a', true)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_with_columns() {
    let res = parse(
        "insert into t (a, b, c) values (1, 2, 3)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_qualified_table_with_alias() {
    let res = parse(
        "insert into myschema.mytable as t (a) values (1)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_trailing_semicolon() {
    let res = parse("insert into t values (1);", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_multiple_insert_and_update_statements() {
    let res = parse(
        "insert into t values (1); update t set a = 2;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_select() {
    let res = parse(
        "insert into t select a, b from u where a > 1",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_select_with_columns_and_trailing_semicolon() {
    let res = parse(
        "insert into t (a, b) select a, b from u;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_multiple_insert_select_statements() {
    let res = parse(
        "insert into t select a from u; insert into t values (1);",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
