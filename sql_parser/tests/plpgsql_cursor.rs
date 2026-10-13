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
fn test_cursor_declarations() {
    let src = r#"do $$
DECLARE
    trigger_cursor CURSOR FOR select tgname from pg_trigger where tgrelid = 1;
    c2 no scroll cursor is select 1;
    c3 scroll cursor for select * from ~Таблица~;
    _cursor refcursor;
    x int := 0;
BEGIN
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_CURSOR_DECLARATION"), 3);
    assert_eq!(count(src, "PSQL_PL_SCROLL_OPTION"), 2);
    assert_eq!(count(src, "PSQL_PL_VARIABLE_DECLARATION"), 2);
}

#[test]
fn test_open_fetch_close() {
    let src = r#"do $$
declare
    _cursor refcursor;
begin
    open trigger_cursor;
    open bound_cursor(1, 'a');
    open doc_cursor for select "Поле", b from t where a > 1;
    open _cursor scroll for execute format('select * from %I', tbl) using x;
    loop
        fetch doc_cursor into idDoc, idDog;
        fetch next from _cursor into _rid;
        fetch absolute 5 from _cursor into _rid;
        fetch prior in _cursor into _rid;
        exit when not found;
    end loop;
    move last from _cursor;
    move relative -2 from _cursor;
    close doc_cursor;
    close _cursor;
end $$"#;

    assert_eq!(count(src, "PSQL_PL_OPEN_STATEMENT"), 4);
    assert_eq!(count(src, "PSQL_PL_OPEN_FOR_CLAUSE"), 2);
    assert_eq!(count(src, "PSQL_PL_FETCH_STATEMENT"), 6);
    assert_eq!(count(src, "PSQL_PL_FETCH_DIRECTION"), 5);
    assert_eq!(count(src, "PSQL_PL_CLOSE_STATEMENT"), 2);
}

#[test]
fn test_cursor_keywords_are_still_valid_names() {
    let res = parse(
        "select cursor, scroll, open, close, move, prior, absolute, relative, forward, backward from t",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_broken_cursors_report_errors() {
    for src in [
        "do $$ declare c cursor select 1; begin end $$",
        "do $$ declare c cursor for; begin end $$",
        "do $$ begin open; end $$",
        "do $$ begin open c for; end $$",
        "do $$ begin open c end $$",
        "do $$ begin fetch next from into x; end $$",
        "do $$ begin fetch c into; end $$",
        "do $$ begin close; end $$",
        "do $$ begin close c end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
