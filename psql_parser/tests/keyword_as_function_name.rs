#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_right_as_function_name() {
    let res = parse("select right(name, 4) from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_left_as_function_name() {
    let res = parse("select left(name, 1) from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_right_as_function_name_nested_call() {
    let res = parse(
        "select right('0000' || trim(name), 20) from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_right_join_still_works() {
    let res = parse(
        "select a from t1 right join t2 on t1.id = t2.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_left_join_still_works() {
    let res = parse(
        "select a from t1 left join t2 on t1.id = t2.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_left_outer_join_still_works() {
    let res = parse(
        "select a from t1 left outer join t2 on t1.id = t2.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_bare_right_without_parens_is_still_an_error() {
    // Out of scope -- only `right(`/`left(` (immediately followed by a
    // paren) are treated as a call; a bare `right`/`left` reference stays
    // a diagnostic, consistent with how `full`'s own alias-position
    // collision is deliberately left alone.
    let res = parse("select right from t", PsqlFileSource::script());

    assert!(res.has_errors());
}
