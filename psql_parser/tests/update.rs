#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_update_single_set_item() {
    let res = parse("update t set a = 1", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_update_multiple_set_items() {
    let res = parse("update t set a = 1, b = a + 1", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_update_set_target_named_after_type_keyword() {
    // Same `full`-class collision, for a `SET` target column.
    let res = parse("update t set date = now()", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_update_with_where_clause() {
    let res = parse("update t set a = 1 where b > 2", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_update_qualified_table_with_alias() {
    let res = parse(
        "update myschema.mytable as t set a = 1 where t.b > 2",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_trailing_semicolon() {
    let res = parse("update t set a = 1;", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_multiple_update_and_delete_statements() {
    let res = parse(
        "update t set a = 1; delete from t where a = 0;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_with_from_clause() {
    let res = parse(
        "update t set a = u.a from u where t.id = u.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_with_from_clause_multiple_items() {
    let res = parse(
        "update t set a = u.a from u, v where t.id = u.id and u.v_id = v.id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_from_clause_with_returning() {
    let res = parse(
        "update t set a = u.a from u where t.id = u.id returning t.id;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_update_from_clause_without_where() {
    let res = parse("update t set a = u.a from u;", PsqlFileSource::script());

    assert_parser!(res);
}
