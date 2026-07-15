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
