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
fn test_get_diagnostics() {
    let src = r#"do $$
declare
    row_count int;
BEGIN
    delete from ~Таблица~ where "Ключ" = 1;
    GET DIAGNOSTICS row_count = ROW_COUNT;
    get current diagnostics row_count := row_count, ctx = pg_context;
    get stacked diagnostics msg = message_text, st = returned_sqlstate;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_GET_DIAGNOSTICS_STATEMENT"), 3);
    assert_eq!(count(src, "PSQL_PL_DIAGNOSTICS_ITEM"), 5);
}

#[test]
fn test_get_is_still_a_valid_name() {
    let res = parse(
        "do $$ begin get := 1; select get, diagnostics, stacked from t; end $$",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_broken_get_diagnostics_reports_errors() {
    for src in [
        "do $$ begin get diagnostics; end $$",
        "do $$ begin get diagnostics x row_count; end $$",
        "do $$ begin get diagnostics x = ; end $$",
        "do $$ begin get stacked x = row_count; end $$",
        "do $$ begin get diagnostics x = row_count end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
