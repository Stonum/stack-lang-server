#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_select_literal_list() {
    let res = parse(
        "select *, 1, 2, true, false, 'str', 1.15",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_asliased_literals() {
    let res = parse(
        "select 1 as x, 2 as y, 3 z, 4 \"as\"",
        PsqlFileSource::script(),
    );

    assert_parser!(res, debug);
}

#[test]
fn test_select_binary_expressions() {
    let res = parse(
        "select 1 + 2, 3 - 4, 5 * 6, 7 / 8, 9 % 10, 1 < 2, 1 <= 2, 1 > 2, 1 >= 2, 1 = 2, 1 != 2, 1 <> 2",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_logical_expressions() {
    let res = parse(
        "select a and b, a or b, a and b or c",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_binary_expression_precedence() {
    let res = parse(
        "select 1 + 2 * 3, (1 + 2) * 3, a or b and c, a = 1 and b = 2",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_col_references() {
    let res = parse("select a, t.a, s.t.a, d.s.t.a", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_col_reference_with_implicit_alias() {
    let res = parse("select t.a b, t.a as b", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_from_table() {
    let res = parse("select a from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_from_qualified_table_with_alias() {
    let res = parse(
        "select t.a from mydb.myschema.mytable as t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_from_function_binding() {
    let res = parse(
        "select a from some_func(1, 2) as t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_from_qualified_function_binding() {
    let res = parse(
        "select a from myschema.some_func(1, x) t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_where_clause() {
    let res = parse(
        "select a from t where a = 1 and b > 2",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_where_clause_without_from() {
    let res = parse("select 1 where true", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_group_by_clause() {
    let res = parse(
        "select a, b from t group_by a, b,",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_having_clause() {
    let res = parse(
        "select a, b from t group_by a having b > 1",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_order_by_clause() {
    let res = parse(
        "select a, b from t order_by a asc, b desc, a + b",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_order_by_after_having() {
    let res = parse(
        "select a, b from t group_by a having b > 1 order_by a desc",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
