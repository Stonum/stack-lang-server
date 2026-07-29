#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_create_table_single_column() {
    let res = parse("create table foo (a int)", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_create_table_multiple_columns() {
    let res = parse(
        "create table foo (a int, b text, c boolean)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_if_not_exists() {
    let res = parse(
        "create table if not exists foo (a int);",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_qualified_name() {
    let res = parse(
        "create table myschema.foo (a int)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_typed_column_with_arguments() {
    let res = parse(
        "create table foo (a numeric(10, 2))",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_followed_by_another_statement() {
    let res = parse(
        "create table foo (a int); select a from foo;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_tilde_name_in_mlang_dialect() {
    let res = parse("create table ~Договор~ (a int)", mlang());

    assert_parser!(res);
}

#[test]
fn test_create_and_drop_table_do_not_shadow_each_other() {
    let res = parse(
        "drop table if exists foo; create table foo (a int);",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
