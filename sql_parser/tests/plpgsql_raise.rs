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
fn test_raise_forms() {
    let src = r#"do $$
BEGIN
    raise notice '% Обновлено записей: %', now(), cnt;
    RAISE EXCEPTION 'Ошибка' USING ERRCODE = '27101';
    raise exception using message = 'x', hint = 'y', column = "Поле", table = 't';
    raise division_by_zero;
    raise sqlstate '22012';
    raise 'no level';
    raise;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_RAISE_STATEMENT"), 7);
    assert_eq!(count(src, "PSQL_PL_RAISE_OPTION"), 5);
}

#[test]
fn test_raise_levels() {
    let src = "do $$ begin raise debug 'a'; raise log 'a'; raise info 'a'; raise notice 'a'; raise warning 'a'; raise exception 'a'; end $$";

    assert_eq!(count(src, "PSQL_PL_RAISE_STATEMENT"), 6);
}

#[test]
fn test_raise_keywords_are_still_valid_names() {
    let res = parse(
        "select log(10, 2), notice, warning, info, debug, sqlstate, raise, exception from t",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_broken_raise_reports_errors() {
    for src in [
        "do $$ begin raise notice 'a' end $$",
        "do $$ begin raise sqlstate; end $$",
        "do $$ begin raise exception 'a' using; end $$",
        "do $$ begin raise exception 'a' using errcode; end $$",
        "do $$ begin raise exception 'a' using errcode = '1',; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
