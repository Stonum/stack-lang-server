#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_in() {
    let res = parse(
        "select a from t where a in (1, 2, 3)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_in() {
    let res = parse(
        "select a from t where a not in (1, 2, 3)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_in_empty_list() {
    let res = parse("select a from t where a in ()", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_in_chained_with_and() {
    let res = parse(
        "select a from t where a in (1, 2) and b",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_not_binds_looser_than_in() {
    let res = parse(
        "select a from t where not a in (1, 2)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_in_with_expression_items() {
    let res = parse(
        "select a from t where a in (b + 1, upper(c))",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}
