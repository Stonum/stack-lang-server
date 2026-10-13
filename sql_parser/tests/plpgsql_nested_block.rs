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
fn test_null_statements() {
    let src = r#"do $$
BEGIN
    NULL;
    if x then
        null;
    else
        delete from ~Таблица~;
    end if;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_NULL_STATEMENT"), 2);
}

#[test]
fn test_nested_blocks() {
    let src = r#"do $$
declare
    x int := 1;
begin
    begin
        null;
    end;
    declare
        y int := x;
    begin
        <<inner_block>>
        begin
            y := y + 1;
        end inner_block;
    end;
end $$"#;

    assert_eq!(count(src, "PSQL_PL_BLOCK"), 4);
    assert_eq!(count(src, "PSQL_PL_DECLARE_SECTION"), 2);
}

#[test]
fn test_broken_null_statement_reports_errors() {
    for src in ["do $$ begin null end $$", "do $$ begin null null; end $$"] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
