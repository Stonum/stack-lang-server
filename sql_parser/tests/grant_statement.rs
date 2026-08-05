#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mlang() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Mlang)
}

#[test]
fn test_grant_all_on_table_to_public() {
    let res = parse("grant all on table foo to public", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_grant_without_table_keyword() {
    let res = parse("grant all on foo to public", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_grant_multiple_objects_and_grantees() {
    let res = parse(
        "grant all on table foo, bar to public, other_role;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_grant_trailing_semicolon() {
    let res = parse("grant all on table foo to public;", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_grant_followed_by_another_statement() {
    let res = parse(
        "grant all on table foo to public; select a from t;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_grant_tilde_name_in_mlang_dialect() {
    let res = parse("grant all on table ~t~ to public;", mlang());

    assert_parser!(res);
}

#[test]
fn test_realistic_grant_shape() {
    // Representative of the real corpus shape: a view followed by a
    // blanket `GRANT ALL ... TO public`.
    let res = parse(
        r#"create view ~$some_view~ as select a from t;
GRANT ALL ON TABLE ~$some_view~ TO public;"#,
        mlang(),
    );

    assert_parser!(res);
}
