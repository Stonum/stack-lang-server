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
fn test_return_forms() {
    let src = r#"do $$
BEGIN
    return null;
    RETURN;
    return new;
    return (select count(*) from ~Таблица~);
    return next rec;
    return next;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_RETURN_STATEMENT"), 6);
    assert_eq!(count(src, "NEXT_KW"), 2);
}

#[test]
fn test_return_query() {
    let src = r#"do $$
begin
    RETURN QUERY SELECT generate_series(1, 10);
    return query with a as (select 1) select * from a;
    return query execute format('select * from %I', tbl) using x;
end $$"#;

    assert_eq!(count(src, "PSQL_PL_RETURN_QUERY_STATEMENT"), 3);
    assert_eq!(count(src, "PSQL_PL_EXECUTE_STATEMENT"), 1);
}

#[test]
fn test_return_keywords_are_still_valid_names() {
    let res = parse("select query, \"return\" from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_return_reports_errors() {
    for src in [
        "do $$ begin return 1 end $$",
        "do $$ begin return query; end $$",
        "do $$ begin return next 1 2; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
