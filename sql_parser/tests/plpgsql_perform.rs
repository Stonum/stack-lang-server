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
fn test_perform_call() {
    let src = r#"do $$
BEGIN
    PERFORM pg_notify(current_database() || '.log', '{}'::text);
    perform process_table(rec."Таблица", rec."Поле");
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_PERFORM_STATEMENT"), 2);
}

#[test]
fn test_perform_query() {
    let src = "do $$ begin perform a, b from t join t2 on t2.id = t.id where t.a > 1; end $$";

    assert_eq!(count(src, "PSQL_PL_PERFORM_STATEMENT"), 1);
    assert_eq!(count(src, "SQL_WHERE_CLAUSE"), 1);
}

#[test]
fn test_perform_is_still_a_valid_name() {
    let res = parse("select perform from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_perform_reports_errors() {
    for src in [
        "do $$ begin perform; end $$",
        "do $$ begin perform f() end $$",
        "do $$ begin perform from t; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
