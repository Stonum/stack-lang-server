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

#[test]
fn test_create_table_null_and_not_null_columns() {
    let res = parse(
        "create table #t (a int not null, b varchar(256) null, c int)",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_NOT_NULL_CONSTRAINT"));
    assert!(tree.contains("SQL_NULL_CONSTRAINT"));
}

#[test]
fn test_create_table_default_values() {
    let res = parse(
        "create table t (a int default 0 not null, b timestamp default now(), c text not null default 'x' || 'y')",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_DEFAULT_CONSTRAINT").count(), 3);
}

#[test]
fn test_create_table_default_without_value_reports_error() {
    let res = parse("create table t (a int default)", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_table_dangling_not_reports_error() {
    let res = parse("create table t (a int not)", postgres());

    assert!(res.has_errors());
}
