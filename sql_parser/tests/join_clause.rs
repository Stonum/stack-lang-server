#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

/// `LATERAL` and `JOIN ... USING` are both Postgres-only.
fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

#[test]
fn test_bare_join() {
    let res = parse(
        "select a from t join u on t.id = u.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_inner_join() {
    let res = parse(
        "select a from t inner join u on t.id = u.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_left_join() {
    let res = parse(
        "select a from t left join u on t.id = u.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_left_outer_join() {
    let res = parse(
        "select a from t left outer join u on t.id = u.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_right_join() {
    let res = parse(
        "select a from t right join u on t.id = u.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_multiple_joins() {
    let res = parse(
        "select a from t join u on t.id = u.id join v on u.id = v.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_join_with_aliases_and_where() {
    let res = parse(
        "select a from t as x join u as y on x.id = y.id where x.a > 1",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_join_function_binding() {
    let res = parse(
        "select a from t join some_func(1) as f on t.id = f.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_full_join() {
    let res = parse(
        "select a from t full join u on t.id = u.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_full_outer_join() {
    let res = parse(
        "select a from t full outer join u on t.id = u.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_cross_join() {
    let res = parse("select a from t cross join u", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cross_join_then_inner_join() {
    let res = parse(
        "select a from t cross join u join v on u.id = v.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_join_lateral_subquery() {
    let res = parse(
        "select a from t join lateral (select max(b) from u where u.t_id = t.id) x on true",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_left_join_lateral_function() {
    let res = parse(
        "select a from t left join lateral unnest(t.arr) x on true",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_join_lateral_qualified_function() {
    let res = parse(
        "select a from t join lateral myschema.some_func(t.id) x on true",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_join_using_single_column() {
    let res = parse("select a from t join u using (id)", postgres());

    assert_parser!(res);
}

#[test]
fn test_join_using_multiple_columns() {
    let res = parse("select a from t join u using (id, name)", postgres());

    assert_parser!(res);
}

#[test]
fn test_left_join_using() {
    let res = parse("select a from t left join u using (id)", postgres());

    assert_parser!(res);
}

#[test]
fn test_multiple_joins_using() {
    let res = parse(
        "select a from t join u using (id) join v using (id)",
        postgres(),
    );

    assert_parser!(res);
}
