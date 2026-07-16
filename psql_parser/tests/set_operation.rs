#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_union() {
    let res = parse(
        "select a from t union select b from u",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_union_all() {
    let res = parse(
        "select a from t union all select b from u",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_union_distinct() {
    let res = parse(
        "select a from t union distinct select b from u",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_intersect() {
    let res = parse(
        "select a from t intersect select a from u",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_except() {
    let res = parse(
        "select a from t except select a from u",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_chained_set_operations() {
    let res = parse(
        "select a from t union select a from u intersect select a from v",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_union_with_where_and_group_by_on_both_sides() {
    let res = parse(
        "select a from t where a > 1 group_by a union select b from u where b > 2 group_by b",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_union_with_trailing_order_by_limit_offset() {
    let res = parse(
        "select a from t union select a from u order_by a limit 5 offset 1",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_union_without_from_clause() {
    let res = parse("select 1 union select 2", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_union_inside_subquery() {
    let res = parse(
        "select a from (select a from t union select a from u) as sub",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_union_in_insert_select() {
    let res = parse(
        "insert into archive select a from t union select a from u",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_union_trailing_semicolon() {
    let res = parse(
        "select a from t union select a from u;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
