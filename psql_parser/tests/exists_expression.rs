#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_exists_with_subquery() {
    let res = parse(
        "select a from t where exists (select 1 from u where u.a = t.a)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_exists_with_subquery() {
    let res = parse(
        "select a from t where not exists (select 1 from u where u.a = t.a)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_exists_combined_with_or() {
    let res = parse(
        "select a from t where exists (select 1 from u where u.a = t.a) or a = 1",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_policy_using_exists_subquery() {
    // Representative of the real shape this was fixed for: an `EXISTS
    // (subquery)` predicate combined with other conditions through `OR`,
    // inside a `CREATE POLICY ... USING (...)` clause.
    let res = parse(
        r#"create policy p on ~t~ for all using
(
   exists (select 1 from ~u~ where "Some Column" = current_query()) or ("Flag" = 0)
);"#,
        mlang(),
    );

    assert_parser!(res);
}
