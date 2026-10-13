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
fn test_for_over_ranges() {
    let src = r#"do $$
BEGIN
    FOR i IN 1..10 LOOP
        delete from ~Таблица~ where "Ключ" = i;
    END LOOP;
    for i in 0..bufLen-1 loop end loop;
    for i in reverse 10 .. 1 by 2 loop end loop;
    for i in (a + 1)..array_length(arr, 1) by step loop end loop;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_FOR_RANGE_SOURCE"), 4);
    assert_eq!(count(src, "PSQL_PL_FOR_RANGE_STEP"), 2);
    assert_eq!(count(src, "REVERSE_KW"), 1);
}

#[test]
fn test_parenthesized_query_is_not_a_range() {
    let src = "do $$ begin for r in (select 1) loop end loop; end $$";

    assert_eq!(count(src, "PSQL_PL_FOR_RANGE_SOURCE"), 0);
    assert_eq!(count(src, "SQL_SUBQUERY_EXPRESSION"), 1);
}

#[test]
fn test_reverse_is_still_a_function_name() {
    let res = parse("select reverse(name), 1.5, 2. from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_ranges_report_errors() {
    for src in [
        "do $$ begin for i in 1.. loop end loop; end $$",
        "do $$ begin for i in ..10 loop end loop; end $$",
        "do $$ begin for i in 1 10 loop end loop; end $$",
        "do $$ begin for i in 1..10 by loop end loop; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
