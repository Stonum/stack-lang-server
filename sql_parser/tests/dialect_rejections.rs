//! Systematic sweep proving every `SqlSyntaxFeature::Postgres`-gated
//! construct is rejected under `Standard` (and, for a representative
//! sample, explicitly under `Mssql` too -- both dialects go through the
//! same `dialect().is_postgres()` check, so one dialect standing in for
//! both everywhere would be redundant, but a handful of explicit `Mssql`
//! checks confirm that's actually true rather than assumed). The
//! accept-side (Postgres parses each of these cleanly) is already covered
//! by each construct's own dedicated test file -- this file exists only to
//! fill the reject-side gap left by those files, not to duplicate them.

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mssql() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Mssql)
}

#[test]
fn test_returning_rejected_under_standard_dialect() {
    let res = parse(
        "insert into t values (1) returning *",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_returning_rejected_under_mssql_dialect() {
    let res = parse("insert into t values (1) returning *", mssql());

    assert!(res.has_errors());
}

#[test]
fn test_on_conflict_rejected_under_standard_dialect() {
    let res = parse(
        "insert into t values (1) on conflict do nothing",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_lateral_rejected_under_standard_dialect() {
    let res = parse(
        "select a from t, lateral generate_series(1, t.n) g",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_lateral_rejected_under_mssql_dialect() {
    let res = parse(
        "select a from t, lateral generate_series(1, t.n) g",
        mssql(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_join_using_rejected_under_standard_dialect() {
    let res = parse("select a from t join u using (id)", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_distinct_on_rejected_under_standard_dialect() {
    let res = parse(
        "select distinct on (a) a, b from t",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_limit_rejected_under_standard_dialect() {
    let res = parse("select a from t limit 10", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_limit_rejected_under_mssql_dialect() {
    let res = parse("select a from t limit 10", mssql());

    assert!(res.has_errors());
}

#[test]
fn test_array_subscript_rejected_under_standard_dialect() {
    let res = parse("select a[1] from t", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_filter_clause_rejected_under_standard_dialect() {
    let res = parse(
        "select count(x) filter (where a > 1) from t",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_substring_from_for_rejected_under_standard_dialect() {
    let res = parse("select substring(str from 3)", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_create_policy_rejected_under_standard_dialect() {
    let res = parse("create policy p on t", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_create_trigger_rejected_under_standard_dialect() {
    let res = parse(
        "create trigger t before insert on foo execute function f()",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_create_function_rejected_under_standard_dialect() {
    let res = parse(
        "create function foo() as 'select 1'",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_create_function_rejected_under_mssql_dialect() {
    let res = parse("create function foo() as 'select 1'", mssql());

    assert!(res.has_errors());
}
