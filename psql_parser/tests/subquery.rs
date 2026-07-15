#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_subquery_as_from_source() {
    let res = parse(
        "select a from (select a from t) as sub",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_subquery_as_from_source_without_alias() {
    let res = parse("select a from (select a from t)", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_subquery_joined_with_table() {
    let res = parse(
        "select a from t join (select id from u) as sub on t.id = sub.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_scalar_subquery_in_select_list() {
    let res = parse(
        "select (select count(a) from t) as cnt from u",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_scalar_subquery_in_where_clause() {
    let res = parse(
        "select a from t where a = (select max(a) from u)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_in_subquery() {
    let res = parse(
        "select a from t where a in (select id from u)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_in_subquery() {
    let res = parse(
        "select a from t where a not in (select id from u)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_in_value_list_still_works() {
    let res = parse(
        "select a from t where a in (1, 2, 3)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_nested_subquery() {
    let res = parse(
        "select a from (select a from (select a from t) as inner_t) as outer_t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_delete_using_subquery() {
    let res = parse(
        "delete from t using (select id from u) as sub where t.id = sub.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
