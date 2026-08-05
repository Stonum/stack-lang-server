#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mlang() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

#[test]
fn test_drop_table_bare_name() {
    let res = parse("drop table foo", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_table_if_exists() {
    let res = parse("drop table if exists foo;", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_table_qualified_name() {
    let res = parse("drop table myschema.foo", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_table_multiple_names() {
    let res = parse("drop table foo, bar, baz", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_table_cascade() {
    let res = parse("drop table if exists foo cascade;", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_table_restrict() {
    let res = parse(
        "drop table if exists foo restrict;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_table_followed_by_another_statement() {
    let res = parse(
        "drop table if exists foo; select a from t;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_table_tilde_name_in_mlang_dialect() {
    let res = parse("drop table if exists ~Договор~", mlang());

    assert_parser!(res);
}

#[test]
fn test_drop_function_still_dispatches_correctly_next_to_drop_table() {
    // Regression check for the `drop` dispatcher: `DROP TABLE` and `DROP
    // FUNCTION`/`PROCEDURE` must each route to their own grammar, not
    // shadow one another, when they appear back to back.
    let res = parse(
        "drop table if exists foo; drop function if exists bar;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}
