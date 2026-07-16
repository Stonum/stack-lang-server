#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_bare_join() {
    let res = parse(
        "select a from t join u on t.id = u.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_inner_join() {
    let res = parse(
        "select a from t inner join u on t.id = u.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_left_join() {
    let res = parse(
        "select a from t left join u on t.id = u.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_left_outer_join() {
    let res = parse(
        "select a from t left outer join u on t.id = u.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_right_join() {
    let res = parse(
        "select a from t right join u on t.id = u.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_multiple_joins() {
    let res = parse(
        "select a from t join u on t.id = u.id join v on u.id = v.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_join_with_aliases_and_where() {
    let res = parse(
        "select a from t as x join u as y on x.id = y.id where x.a > 1",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_join_function_binding() {
    let res = parse(
        "select a from t join some_func(1) as f on t.id = f.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_full_join() {
    let res = parse(
        "select a from t full join u on t.id = u.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_full_outer_join() {
    let res = parse(
        "select a from t full outer join u on t.id = u.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_cross_join() {
    let res = parse("select a from t cross join u", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_cross_join_then_inner_join() {
    let res = parse(
        "select a from t cross join u join v on u.id = v.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
