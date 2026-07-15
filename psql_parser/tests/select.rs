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
