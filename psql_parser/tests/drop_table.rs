#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_drop_table_bare_name() {
    let res = parse("drop table foo", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_table_if_exists() {
    let res = parse("drop table if exists foo;", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_table_qualified_name() {
    let res = parse("drop table myschema.foo", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_table_multiple_names() {
    let res = parse("drop table foo, bar, baz", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_table_cascade() {
    let res = parse(
        "drop table if exists foo cascade;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_table_restrict() {
    let res = parse(
        "drop table if exists foo restrict;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_table_followed_by_another_statement() {
    let res = parse(
        "drop table if exists foo; select a from t;",
        PsqlFileSource::script(),
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
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
