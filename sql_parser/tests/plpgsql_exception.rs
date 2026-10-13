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
fn test_exception_handlers() {
    let src = r#"do $$
BEGIN
    insert into ~Таблица~ ("Ключ") values (1);
EXCEPTION
    WHEN duplicate_table THEN
    WHEN check_violation OR unique_violation THEN
        raise notice 'skipped';
    WHEN SQLSTATE '22P02' THEN
        get stacked diagnostics msg = message_text;
    WHEN others THEN
        null;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_EXCEPTION_SECTION"), 1);
    assert_eq!(count(src, "PSQL_PL_EXCEPTION_HANDLER"), 4);
    assert_eq!(count(src, "PSQL_PL_SQLSTATE_CONDITION"), 1);
    assert_eq!(count(src, "OR_KW"), 1);
}

#[test]
fn test_nested_block_with_exception() {
    let src = r#"do $$
declare
    x int;
begin
    begin
        x := 1 / 0;
    exception when division_by_zero then
        x := 0;
    end;
    loop
        begin
            exit;
        exception when others then
            return;
        end;
    end loop;
exception
    when no_data_found then
end $$"#;

    assert_eq!(count(src, "PSQL_PL_EXCEPTION_SECTION"), 3);
}

#[test]
fn test_raise_exception_is_not_a_section() {
    let src = "do $$ begin raise exception 'x'; exception when others then null; end $$";

    assert_eq!(count(src, "PSQL_PL_RAISE_STATEMENT"), 1);
    assert_eq!(count(src, "PSQL_PL_EXCEPTION_SECTION"), 1);
}

#[test]
fn test_broken_exception_sections_report_errors() {
    for src in [
        "do $$ begin exception end $$",
        "do $$ begin exception when then null; end $$",
        "do $$ begin exception when others null; end $$",
        "do $$ begin exception when a or then null; end $$",
        "do $$ begin exception when sqlstate then null; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
