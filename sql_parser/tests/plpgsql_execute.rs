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
fn test_execute_forms() {
    let src = r#"do $$
BEGIN
    execute format('truncate table %s', tmp_name);
    EXECUTE _body;
    execute 'insert into ' || target_table || ' select * from ~Таблица~';
    execute format($exec$ select count(*) from %I $exec$, tbl) into cnt;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_EXECUTE_STATEMENT"), 4);
    assert_eq!(count(src, "PSQL_INTO_CLAUSE"), 1);
}

#[test]
fn test_execute_into_using() {
    let src = r#"do $$
begin
    execute query_text into strict result_value using owner_id, (old->>'Поле')::int;
    execute query_text using owner_id;
end $$"#;

    assert_eq!(count(src, "PSQL_PL_EXECUTE_STATEMENT"), 2);
    assert_eq!(count(src, "PSQL_PL_USING_CLAUSE"), 2);
    assert_eq!(count(src, "PSQL_INTO_CLAUSE"), 1);
}

#[test]
fn test_broken_execute_reports_errors() {
    for src in [
        "do $$ begin execute; end $$",
        "do $$ begin execute q into; end $$",
        "do $$ begin execute q using; end $$",
        "do $$ begin execute q end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
