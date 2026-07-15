#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_searched_case() {
    let res = parse(
        "select case when a = 1 then 'one' when a = 2 then 'two' else 'other' end from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_simple_case() {
    let res = parse(
        "select case a when 1 then 'one' when 2 then 'two' else 'other' end from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_case_without_else() {
    let res = parse(
        "select case when a = 1 then 'one' end from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_case_single_when() {
    let res = parse(
        "select case when a then b end from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_case_in_where_clause() {
    let res = parse(
        "select a from t where case when a then true else false end",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_nested_case() {
    let res = parse(
        "select case when a then case when b then 1 else 2 end else 3 end from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_case_with_alias() {
    let res = parse(
        "select case when a then 1 end as result from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
