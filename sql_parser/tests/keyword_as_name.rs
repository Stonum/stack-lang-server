#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

#[test]
fn test_non_reserved_keywords_as_column_references() {
    let res = parse("select key, rows, text, date from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_non_reserved_keywords_as_qualified_column_references() {
    let res = parse("select t.key, t.rows from t where t.text = 'a'", postgres());

    assert_parser!(res);
}

#[test]
fn test_non_reserved_keywords_in_where_and_order_by() {
    let res = parse(
        "select a from t where rows > 0 and key is not null order by key desc",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_non_reserved_keyword_as_table_name() {
    let res = parse("select a from key k join rows r on r.id = k.id", postgres());

    assert_parser!(res);
}

#[test]
fn test_non_reserved_keyword_as_function_name() {
    let res = parse("select date(x), replace(a, 'b', 'c') from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_non_reserved_keyword_as_function_parameter_name() {
    let res = parse(
        "create function f(rows text, key int) returns int as $$ select 1 $$ language sql",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_non_reserved_keywords_as_create_table_columns() {
    let res = parse("create table t (key int, rows text)", postgres());

    assert_parser!(res);
}

#[test]
fn test_non_reserved_keyword_as_update_target_column() {
    let res = parse("update t set key = 1, rows = 2", postgres());

    assert_parser!(res);
}

#[test]
fn test_reserved_keyword_as_alias_after_as() {
    let res = parse("select a as table, b as select from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_reserved_keyword_is_still_not_a_column_reference() {
    let res = parse("select from from t", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_full_join_is_still_a_join_not_an_alias() {
    let res = parse("select a from t full join u on true", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_JOIN_CLAUSE"));
}

#[test]
fn test_fetch_first_rows_only_still_parses() {
    let res = parse("select a from t fetch first rows only", postgres());

    assert_parser!(res);
}

#[test]
fn test_parenthesized_values_in_from_is_still_a_subquery() {
    let res = parse("select * from (values (1)) as v(a)", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(!tree.contains("SQL_PARENTHESIZED_JOIN_BINDING"));
}

#[test]
fn test_non_reserved_keywords_as_bare_aliases() {
    for src in [
        "select work.a from t work join u lock on lock.id = work.id",
        "select a key, b text from t",
        "select a from (select 1 as a) range",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("SQL_ALIAS"), "{src}");
    }
}

#[test]
fn test_bare_alias_stops_at_continuing_keywords() {
    for src in [
        "update t set a = 1",
        "insert into t values (1)",
        "select count(*) filter (where a > 0) over (partition by b) from t",
        "select a from t\nGO\nselect b from u",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
    }
}
