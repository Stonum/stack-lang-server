#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_insert_returning_star() {
    let res = parse(
        "insert into t values (1) returning *",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_insert_returning_columns() {
    let res = parse(
        "insert into t (a, b) values (1, 2) returning a, b as bb",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_returning() {
    let res = parse(
        "update t set a = 1 where id = 1 returning a",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_delete_returning() {
    let res = parse(
        "delete from t where id = 1 returning *",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_returning_with_trailing_semicolon() {
    let res = parse(
        "insert into t values (1) returning *;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}
