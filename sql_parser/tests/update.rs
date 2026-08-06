#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

/// `RETURNING` is Postgres-only.
fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

#[test]
fn test_update_single_set_item() {
    let res = parse("update t set a = 1", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_update_multiple_set_items() {
    let res = parse("update t set a = 1, b = a + 1", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_update_set_target_named_after_type_keyword() {
    // Same `full`-class collision, for a `SET` target column.
    let res = parse("update t set date = now()", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_update_with_where_clause() {
    let res = parse("update t set a = 1 where b > 2", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_update_qualified_table_with_alias() {
    let res = parse(
        "update myschema.mytable as t set a = 1 where t.b > 2",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_trailing_semicolon() {
    let res = parse("update t set a = 1;", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_multiple_update_and_delete_statements() {
    let res = parse(
        "update t set a = 1; delete from t where a = 0;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_with_from_clause() {
    let res = parse(
        "update t set a = u.a from u where t.id = u.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_with_from_clause_multiple_items() {
    let res = parse(
        "update t set a = u.a from u, v where t.id = u.id and u.v_id = v.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_from_clause_with_returning() {
    let res = parse(
        "update t set a = u.a from u where t.id = u.id returning t.id;",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_from_clause_without_where() {
    let res = parse("update t set a = u.a from u;", SqlFileSource::script());

    assert_parser!(res);
}
