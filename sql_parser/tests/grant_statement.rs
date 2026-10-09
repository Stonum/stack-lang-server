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

fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

#[test]
fn test_grant_privilege_forms() {
    for src in [
        "grant select, insert, update, delete on table t to r",
        "grant all privileges on table t to public",
        "grant select (a, b), update (a) on t to r with grant option;",
        "grant usage on schema s to r",
        "grant execute on function f, procedure_name to public",
        "grant usage, select on sequence s to r",
        "grant connect, temporary on database d to r",
        "grant usage on language plpgsql to r",
        "grant select on all tables in schema s1, s2 to r",
        "grant execute on all functions in schema s to r",
        "grant some_role, other_role to r",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("SQL_GRANT_STATEMENT"), "{src}");
    }
}

#[test]
fn test_revoke_forms() {
    for src in [
        r#"revoke all privileges on schema "s" from "u";"#,
        "revoke all privileges on all sequences in schema s from u",
        "revoke all privileges on all procedures in schema s from u",
        "revoke all privileges on database d from u",
        "revoke grant option for select on t from r cascade",
        "revoke first_role, second_role from u restrict;",
        "REVOKE ALL PRIVILEGES ON LANGUAGE plpgsql FROM u;",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("SQL_REVOKE_STATEMENT"), "{src}");
    }
}

#[test]
fn test_grant_and_revoke_batch_without_semicolon() {
    let res = parse(
        "revoke all on t from public\ngrant select on t to r\nselect a from t",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_REVOKE_STATEMENT"));
    assert!(tree.contains("SQL_GRANT_STATEMENT"));
    assert!(tree.contains("SQL_SELECT_STATEMENT"));
}

#[test]
fn test_privilege_keywords_are_still_valid_names() {
    let res = parse(
        "select usage, tables, functions, privileges, option from routines",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_grant_without_grantee_reports_error() {
    let res = parse("grant select on t to;", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_grant_all_in_schema_without_kind_reports_error() {
    let res = parse("grant select on all in schema s to r", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_revoke_with_to_instead_of_from_reports_error() {
    let res = parse("revoke select on t to r", postgres());

    assert!(res.has_errors());
}
