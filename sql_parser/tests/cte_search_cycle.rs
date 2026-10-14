#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

fn count(src: &str, kind: &str) -> usize {
    let res = parse(src, postgres());
    assert_parser!(res);
    format!("{:#?}", res.syntax())
        .matches(&format!("{kind}@"))
        .count()
}

#[test]
fn test_cte_cycle_and_search() {
    let src = r#"with recursive tree as (
    select row_id, "Родитель" from ~Таблица~ where "Родитель" is null
    union all
    select t.row_id, t."Родитель" from ~Таблица~ t join tree on t."Родитель" = tree.row_id
) cycle row_id set "is_cycle" using "path"
select * from tree;
with recursive g as (select 1 as id) search depth first by id set ord
    cycle id set is_cycle to true default false using path
select * from g;"#;

    assert_eq!(count(src, "PSQL_CTE_CYCLE_CLAUSE"), 2);
    assert_eq!(count(src, "PSQL_CTE_SEARCH_CLAUSE"), 1);
    assert_eq!(count(src, "PSQL_CTE_CYCLE_MARK_VALUES"), 1);
}

#[test]
fn test_cte_keywords_are_still_valid_names() {
    let res = parse("select search, cycle, depth, breadth from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_cte_clauses_report_errors() {
    for src in [
        "with recursive g as (select 1) cycle id set x select 1",
        "with recursive g as (select 1) cycle id using p select 1",
        "with recursive g as (select 1) search first by id set o select 1",
        "with recursive g as (select 1) cycle id set x to true using p select 1",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
