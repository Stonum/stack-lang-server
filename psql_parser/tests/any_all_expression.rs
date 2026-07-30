#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_any_with_subquery() {
    let res = parse(
        "select a from t where a = any(select b from u)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_all_with_subquery() {
    let res = parse(
        "select a from t where a <> all(select b from u)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_some_with_subquery() {
    let res = parse(
        "select a from t where a = some(select b from u)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_any_with_array_expression() {
    let res = parse(
        "select a from t where a = any(array[1, 2, 3])",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_any_with_plain_parenthesized_expression() {
    let res = parse("select a from t where a = any(b)", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_any_subquery_with_call_expression() {
    let res = parse(
        "select a from t where a = any(select f(g(h())))",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_policy_using_any_subquery() {
    // Representative of the real shape this was fixed for: a quoted column
    // compared via `= ANY (subquery)`, combined with other conditions
    // through `OR`, inside a `CREATE POLICY ... USING (...)` clause.
    let res = parse(
        r#"create policy p on ~t~ for all using
(
   "Some Column" = any(select ~$get_list~('1', ~$get_context~(current_query()))) or ("Flag" = 0) or ("Other Column" is not null)
);"#,
        mlang(),
    );

    assert_parser!(res);
}
