#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_multiple_from_sources() {
    let res = parse(
        "select a from t, u where t.id = u.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_three_from_sources() {
    let res = parse("select a from t, u, v", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_from_source_with_join_followed_by_comma() {
    let res = parse(
        "select a from t join u on t.id = u.id, v",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_multiple_from_sources_with_aliases() {
    let res = parse(
        "select a from t as x, u as y where x.id = y.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_delete_using_multiple_sources() {
    let res = parse(
        "delete from t using u, v where t.id = u.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_from_with_lateral_subquery() {
    let res = parse(
        "select a from t, lateral (select max(b) from u where u.t_id = t.id) x",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_from_with_lateral_function() {
    let res = parse(
        "select a from t, lateral generate_series(1, t.n) g",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_stray_lateral_before_table_name_is_an_error() {
    // `lateral` only makes sense before a subquery or function call --
    // never a plain table name, so this must fail with a normal
    // diagnostic rather than silently accepting `lateral t`.
    let res = parse("select a from t, lateral u", PsqlFileSource::script());

    assert!(res.has_errors());
}
