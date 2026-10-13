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
fn test_declare_section() {
    let res = parse(
        r#"do $$
DECLARE
    _xml xml;
    b_month timestamp := '2024-01-01';
    cnt int = 0;
    "Поле" text NOT NULL DEFAULT 'x';
    k constant int := 1;
    c text collate "C";
    total numeric := (select sum(a) from ~Таблица~);
BEGIN
    delete from t;
END $$;"#,
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("PSQL_PL_VARIABLE_DECLARATION@").count(), 7);
}

#[test]
fn test_nested_block_with_declare() {
    let res = parse(
        "do $$ begin declare x int; begin delete from t; end; end $$",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("PSQL_PL_DECLARE_SECTION@").count(), 1);
}

#[test]
fn test_declare_keywords_are_still_valid_names() {
    let res = parse("select declare, constant from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_declarations_report_errors_and_recover() {
    for src in [
        "do $$ declare x; begin end $$",
        "do $$ declare x int begin end $$",
        "do $$ declare x int := ; y int; begin end $$",
        "do $$ declare x int; $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
