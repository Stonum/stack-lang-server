use crate::helper;
use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

/// `RETURNING` is Postgres-only.
fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

#[test]
fn test_insert_returning_star() {
    let res = parse("insert into t values (1) returning *", postgres());

    assert_parser!(res);
}

#[test]
fn test_insert_returning_columns() {
    let res = parse(
        "insert into t (a, b) values (1, 2) returning a, b as bb",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_returning() {
    let res = parse("update t set a = 1 where id = 1 returning a", postgres());

    assert_parser!(res);
}

#[test]
fn test_delete_returning() {
    let res = parse("delete from t where id = 1 returning *", postgres());

    assert_parser!(res);
}

#[test]
fn test_returning_with_trailing_semicolon() {
    let res = parse("insert into t values (1) returning *;", postgres());

    assert_parser!(res);
}
