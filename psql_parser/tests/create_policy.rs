#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_create_policy_bare() {
    let res = parse("create policy p on t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_create_policy_for_all() {
    let res = parse("create policy p on t for all", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_create_policy_for_select() {
    let res = parse("create policy p on t for select", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_create_policy_for_insert_update_delete() {
    for command in ["insert", "update", "delete"] {
        let src = format!("create policy p on t for {command}");
        let res = parse(&src, PsqlFileSource::script());

        assert_parser!(res);
    }
}

#[test]
fn test_create_policy_using_clause() {
    let res = parse(
        "create policy p on t using (a = 1)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_policy_for_and_using() {
    let res = parse(
        "create policy p on t for all using (a = 1);",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_policy_using_subquery_condition() {
    let res = parse(
        "create policy p on t using (a is null or a = (select f(g(h()))))",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_policy_followed_by_another_statement() {
    let res = parse(
        "create policy p on t for all; select a from t;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_policy_tilde_name_in_mlang_dialect() {
    let res = parse("create policy p on ~t~ for all", mlang());

    assert_parser!(res);
}

#[test]
fn test_drop_policy_bare() {
    let res = parse("drop policy p on t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_policy_if_exists() {
    let res = parse("drop policy if exists p on t;", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_policy_tilde_name_in_mlang_dialect() {
    let res = parse("drop policy if exists p on ~t~;", mlang());

    assert_parser!(res);
}

#[test]
fn test_create_and_drop_policy_do_not_shadow_view_table() {
    let res = parse(
        "drop table if exists foo; drop policy if exists p on foo; create table foo (a int); create policy p on foo for all;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_policy_shape() {
    // Representative of a real row-level-security policy: a `USING`
    // condition combining a null-check with a correlated subquery call.
    let res = parse(
        r#"drop policy if exists rls_read on ~business_process~;
create policy rls_read on ~business_process~ for all using ( owner_id is null or owner_id = ( select ~get_owner_id~(~get_current_user_id~(current_query())) ) );"#,
        mlang(),
    );

    assert_parser!(res);
}
