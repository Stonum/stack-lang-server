#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mlang() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

/// `boolean` is a Postgres-only type keyword.
fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

#[test]
fn test_create_table_single_column() {
    let res = parse("create table foo (a int)", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_create_table_multiple_columns() {
    let res = parse("create table foo (a int, b text, c boolean)", postgres());

    assert_parser!(res);
}

#[test]
fn test_create_table_column_named_full() {
    // `FULL` isn't fully reserved in real Postgres -- it can be used as a
    // plain column name (only its meaning in `FROM ... FULL JOIN`/bare-alias
    // position is special).
    let res = parse("create table foo (full text)", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_create_table_column_named_after_type_keyword() {
    // Same class of collision as `full` above, for a type-name keyword
    // instead (`date timestamp, text_col text` -- both `date` and `text`
    // are used as ordinary column names, followed by their actual type).
    let res = parse(
        "create table foo (date timestamp, text_col text)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_if_not_exists() {
    let res = parse(
        "create table if not exists foo (a int);",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_qualified_name() {
    let res = parse("create table myschema.foo (a int)", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_create_table_typed_column_with_arguments() {
    let res = parse(
        "create table foo (a numeric(10, 2))",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_followed_by_another_statement() {
    let res = parse(
        "create table foo (a int); select a from foo;",
        SqlFileSource::script(),
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
        SqlFileSource::script(),
    );

    assert_parser!(res);
}
