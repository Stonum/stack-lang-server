#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mlang() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Mlang)
}

#[test]
fn test_parenthesized_join_as_outer_join_source() {
    let res = parse(
        "select a from (t1 join t2 on t1.id = t2.id) left join t3 on t2.id = t3.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_parenthesized_join_as_join_target() {
    let res = parse(
        "select a from t1 join (t2 join t3 on t2.id = t3.id) on t1.id = t2.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_nested_parenthesized_join() {
    let res = parse(
        "select a from ((t1 join t2 on t1.id = t2.id) join t3 on t2.id = t3.id)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_parenthesized_join_with_alias() {
    let res = parse(
        "select a from (t1 join t2 on t1.id = t2.id) x",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_parenthesized_join_of_function_bindings() {
    let res = parse(
        "select a from (func1(1) join func2(2) x on true)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_subquery_still_parses_as_subquery_not_parenthesized_join() {
    // Regression guard: both shapes start with `(`, so this only works if
    // the `select`/`with` lookahead actually wins.
    let res = parse("select a from (select b from t) x", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_with_prefixed_subquery_still_parses_as_subquery() {
    let res = parse(
        "select a from (with c as (select 1) select * from c) x",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_left_join_of_parenthesized_tilde_join_with_and_condition() {
    let res = parse(
        r#"select * from t0
left join (
    ~Object~ o
    join ~Object Params~ p on o.id = p.id
) on o.acc = t0.acc"#,
        mlang(),
    );

    assert_parser!(res);
}
