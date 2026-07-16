#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_insert_returning_star() {
    let res = parse(
        "insert into t values (1) returning *",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_returning_columns() {
    let res = parse(
        "insert into t (a, b) values (1, 2) returning a, b as bb",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_returning() {
    let res = parse(
        "update t set a = 1 where id = 1 returning a",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_delete_returning() {
    let res = parse(
        "delete from t where id = 1 returning *",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_returning_with_trailing_semicolon() {
    let res = parse(
        "insert into t values (1) returning *;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
