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
fn test_aggregate_quantifiers() {
    for src in [
        "select count(distinct a) from t",
        r#"select array_agg(DISTINCT t."Поле") from t"#,
        "select sum(all x), count(*) from t",
        "select count(distinct a) filter (where a > 0) from t",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(
            tree.contains("DISTINCT_KW") || tree.contains("ALL_KW"),
            "{src}"
        );
    }
}

#[test]
fn test_aggregate_order_by() {
    for src in [
        "select string_agg(name, ', ' order by name) from t",
        "select string_agg(distinct name, ', ' order by name desc, code) from t",
        "select array_agg(a order by b) from t",
        "select jsonb_agg(t order by t.id) from t",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        let call = tree.find("SQL_CALL_EXPRESSION").expect(src);
        assert!(tree[call..].contains("SQL_ORDER_BY_CLAUSE"), "{src}");
    }
}

#[test]
fn test_window_order_by_is_not_taken_as_aggregate_order_by() {
    let res = parse(
        "select row_number() over (order by a), sum(b) over (partition by c order by d) from t",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_aggregate_quantifier_without_arguments_reports_error() {
    for src in [
        "select count(distinct) from t",
        "select string_agg(a order) from t",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
